use crate::domain::model::project_tree_entity::{ProjectItemEntity, ProjectTreeEntity};
use crate::domain::repository::project_repository::ProjectRepository;
use crate::infra::driver::xml_io_driver::{XmlFiles, XmlIoDriver, XmlItem, XmlProject};
use std::path::PathBuf;

pub struct ProjectXmlRepositoryImpl {
    driver: XmlIoDriver,
}

impl ProjectXmlRepositoryImpl {
    pub fn new(driver: XmlIoDriver) -> Self {
        Self { driver }
    }

    fn to_xml_item(item: &ProjectItemEntity) -> XmlItem {
        match item {
            ProjectItemEntity::File {
                name: _,
                relative_path,
            } => XmlItem::File {
                relative_path: relative_path.to_string_lossy().into_owned(),
            },
            ProjectItemEntity::Folder { name, children, .. } => XmlItem::Filter {
                name: name.clone(),
                children: children.iter().map(Self::to_xml_item).collect(),
            },
        }
    }

    fn to_entity_item(xml_item: &XmlItem) -> ProjectItemEntity {
        match xml_item {
            XmlItem::File { relative_path } => ProjectItemEntity::File {
                name: PathBuf::from(relative_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
                relative_path: PathBuf::from(relative_path),
            },
            XmlItem::Filter { name, children, .. } => ProjectItemEntity::Folder {
                name: name.clone(),
                relative_path: PathBuf::new(), // XMLからは親ディレクトリのみわかるが、ここでは簡易化
                children: children.iter().map(Self::to_entity_item).collect(),
            },
        }
    }

    fn scan_dir_recursive(
        current_path: &PathBuf,
        root_path: &PathBuf,
    ) -> Result<Vec<ProjectItemEntity>, String> {
        let mut items = Vec::new();
        let entries = std::fs::read_dir(current_path).map_err(|e| e.to_string())?;

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();

            if file_name == ".git"
                || file_name == "target"
                || file_name == "node_modules"
                || file_name == ".venv"
            {
                continue;
            }

            let relative_path = path
                .strip_prefix(root_path)
                .map_err(|e| e.to_string())?
                .to_path_buf();

            if path.is_dir() {
                let children = Self::scan_dir_recursive(&path, root_path)?;
                items.push(ProjectItemEntity::Folder {
                    name: file_name,
                    relative_path,
                    children,
                });
            } else {
                // eeproj ファイル自体はプロジェクトから除外する
                if path.extension().and_then(|s| s.to_str()) == Some("eeproj") {
                    continue;
                }
                items.push(ProjectItemEntity::File {
                    name: file_name,
                    relative_path,
                });
            }
        }
        Ok(items)
    }
}

impl ProjectRepository for ProjectXmlRepositoryImpl {
    fn scan(&self, root_path: PathBuf) -> Result<ProjectTreeEntity, String> {
        let items = Self::scan_dir_recursive(&root_path, &root_path)?;
        Ok(ProjectTreeEntity { root_path, items })
    }

    fn save(&self, tree: &ProjectTreeEntity) -> Result<(), String> {
        let project = XmlProject {
            files: XmlFiles {
                items: tree.items.iter().map(Self::to_xml_item).collect(),
            },
        };
        let mut path = tree.root_path.clone();
        path.push("project.eeproj");
        self.driver.write_project(&path, &project)
    }

    fn load(&self, path: PathBuf) -> Result<ProjectTreeEntity, String> {
        let project = self.driver.read_project(&path)?;
        let root_path = path.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();
        let items = project
            .files
            .items
            .iter()
            .map(Self::to_entity_item)
            .collect();
        Ok(ProjectTreeEntity { root_path, items })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::driver::xml_io_driver::XmlIoDriver;
    use std::fs;

    #[test]
    fn test_xml_roundtrip() {
        let driver = XmlIoDriver::new();
        let repo = ProjectXmlRepositoryImpl::new(driver);
        let root = PathBuf::from(".");
        let mut tree = ProjectTreeEntity::new(root.clone());
        tree.add_item(ProjectItemEntity::Folder {
            name: "src".to_string(),
            relative_path: PathBuf::from("src"),
            children: vec![ProjectItemEntity::File {
                name: "main.rs".to_string(),
                relative_path: PathBuf::from("src/main.rs"),
            }],
        });

        repo.save(&tree).unwrap();

        let mut eeproj_path = root.clone();
        eeproj_path.push("project.eeproj");

        let xml_content = fs::read_to_string(&eeproj_path).unwrap();
        println!("Generated XML:\n{}", xml_content);

        let loaded_tree = repo.load(eeproj_path).unwrap();

        assert_eq!(loaded_tree.items.len(), 1);
        if let ProjectItemEntity::Folder { name, .. } = &loaded_tree.items[0] {
            assert_eq!(name, "src");
        } else {
            panic!("Item should be a Folder");
        }

        // Cleanup
        let _ = fs::remove_file("project.eeproj");
    }
}
