use crate::domain::repository::project_repository::ProjectRepository;
use std::path::PathBuf;

pub struct SyncProjectWorkflow<R: ProjectRepository> {
    repository: R,
}

impl<R: ProjectRepository> SyncProjectWorkflow<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// 指定されたルートパスの同期を実行する
    pub fn run(&self, root_path: PathBuf) -> Result<(), String> {
        // 1. スキャン
        let tree = self.repository.scan(root_path)?;

        // 2. 保存 (XML生成)
        self.repository.save(&tree)?;

        // 3. (Optional) EmEditorへのリロード通知 - 後続タスクで実装

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::project_tree_entity::ProjectTreeEntity;

    struct MockRepository;

    impl ProjectRepository for MockRepository {
        fn scan(&self, root_path: PathBuf) -> Result<ProjectTreeEntity, String> {
            Ok(ProjectTreeEntity::new(root_path))
        }

        fn save(&self, _tree: &ProjectTreeEntity) -> Result<(), String> {
            Ok(())
        }

        fn load(&self, path: PathBuf) -> Result<ProjectTreeEntity, String> {
            Ok(ProjectTreeEntity::new(path))
        }
    }

    #[test]
    fn test_sync_workflow_run() {
        let repo = MockRepository;
        let workflow = SyncProjectWorkflow::new(repo);
        let result = workflow.run(PathBuf::from("C:\\project"));

        assert!(result.is_ok());
    }
}
