use std::path::PathBuf;

/// Projectプラグインにおけるファイル・フォルダツリーのエンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectTree {
    pub root_path: PathBuf,
    pub items: Vec<ProjectItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectItem {
    File {
        name: String,
        relative_path: PathBuf,
    },
    Folder {
        name: String,
        relative_path: PathBuf,
        children: Vec<ProjectItem>,
    },
}

impl ProjectTree {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            items: Vec::new(),
        }
    }

    /// アイテムを追加する
    pub fn add_item(&mut self, item: ProjectItem) {
        self.items.push(item);
    }

    /// アイテムを削除する
    pub fn remove_item(&mut self, relative_path: &PathBuf) {
        self.items.retain(|item| match item {
            ProjectItem::File {
                relative_path: p, ..
            } => p != relative_path,
            ProjectItem::Folder {
                relative_path: p, ..
            } => p != relative_path,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_file_item() {
        let mut tree = ProjectTree::new(PathBuf::from("C:\\project"));
        let file = ProjectItem::File {
            name: "main.rs".to_string(),
            relative_path: PathBuf::from("src\\main.rs"),
        };

        tree.add_item(file.clone());

        assert_eq!(tree.items.len(), 1);
        assert_eq!(tree.items[0], file);
    }

    #[test]
    fn test_remove_item() {
        let mut tree = ProjectTree::new(PathBuf::from("C:\\project"));
        let path = PathBuf::from("src\\main.rs");
        let file = ProjectItem::File {
            name: "main.rs".to_string(),
            relative_path: path.clone(),
        };

        tree.add_item(file);
        tree.remove_item(&path);

        assert_eq!(tree.items.len(), 0);
    }
}
