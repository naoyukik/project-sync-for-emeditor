use crate::domain::model::project_tree_entity::{ProjectItemEntity, ProjectTreeEntity};
use crate::domain::repository::project_repository::ProjectRepository;
use crate::infra::driver::xml_io_driver::{XmlIoDriver, XmlItem, XmlProject};
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
                name,
                relative_path,
            } => XmlItem::File {
                name: name.clone(),
                relative_path: relative_path.to_string_lossy().into_owned(),
            },
            ProjectItemEntity::Folder {
                name,
                children,
                ..
            } => XmlItem::Folder {
                name: name.clone(),
                children: children.iter().map(Self::to_xml_item).collect(),
            },
        }
    }

    fn to_entity_item(xml_item: &XmlItem) -> ProjectItemEntity {
        match xml_item {
            XmlItem::File {
                name,
                relative_path,
            } => ProjectItemEntity::File {
                name: name.clone(),
                relative_path: PathBuf::from(relative_path),
            },
            XmlItem::Folder { name, children, .. } => ProjectItemEntity::Folder {
                name: name.clone(),
                relative_path: PathBuf::new(), // XMLからは親ディレクトリのみわかるが、ここでは簡易化
                children: children.iter().map(Self::to_entity_item).collect(),
            },
        }
    }
}

impl ProjectRepository for ProjectXmlRepositoryImpl {
    fn scan(&self, root_path: PathBuf) -> Result<ProjectTreeEntity, String> {
        let tree = ProjectTreeEntity::new(root_path.clone());
        let scanned = self.driver.scan_directory(&root_path)?;

        // 以前のロジックを統合して再構築
        // ここでは以前の scan ロジックを driver に移譲した前提で結果を詰め替える
        for (_path, _is_dir) in scanned {
            // 簡易化のため一旦スキップ、後の Green フェーズで詳細化
        }

        Ok(tree)
    }

    fn save(&self, tree: &ProjectTreeEntity) -> Result<(), String> {
        let project = XmlProject {
            items: tree.items.iter().map(Self::to_xml_item).collect(),
        };
        let mut path = tree.root_path.clone();
        path.push("project.eeproj");
        self.driver.write_project(&path, &project)
    }

    fn load(&self, path: PathBuf) -> Result<ProjectTreeEntity, String> {
        let project = self.driver.read_project(&path)?;
        let root_path = path.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();
        let items = project.items.iter().map(Self::to_entity_item).collect();
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
        tree.add_item(ProjectItemEntity::File {
            name: "test_refactored.rs".to_string(),
            relative_path: PathBuf::from("test_refactored.rs"),
        });

        repo.save(&tree).unwrap();

        let mut eeproj_path = root.clone();
        eeproj_path.push("project.eeproj");
        let loaded_tree = repo.load(eeproj_path).unwrap();

        assert_eq!(loaded_tree.items.len(), 1);
        if let ProjectItemEntity::File { name, .. } = &loaded_tree.items[0] {
            assert_eq!(name, "test_refactored.rs");
        } else {
            panic!("Item should be a File");
        }

        // Cleanup
        let _ = fs::remove_file("project.eeproj");
    }
}
