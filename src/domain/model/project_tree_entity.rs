use std::path::PathBuf;

/// Projectプラグインにおけるファイル・フォルダツリーのエンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectTreeEntity {
    pub root_path: PathBuf,
    pub items: Vec<ProjectItemEntity>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectItemEntity {
    File {
        name: String,
        relative_path: PathBuf,
    },
    Folder {
        name: String,
        relative_path: PathBuf,
        children: Vec<ProjectItemEntity>,
    },
}

impl ProjectTreeEntity {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: ProjectItemEntity) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, relative_path: &PathBuf) {
        self.items.retain(|item| match item {
            ProjectItemEntity::File {
                relative_path: p, ..
            } => p != relative_path,
            ProjectItemEntity::Folder {
                relative_path: p, ..
            } => p != relative_path,
        });
    }
}
