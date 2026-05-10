use crate::domain::project_tree_entity::ProjectTree;
use std::path::PathBuf;

/// プロジェクトの永続化とスキャンのためのインターフェース
pub trait ProjectRepository {
    /// 現在のファイルシステムの状態をスキャンして ProjectTree を生成する
    fn scan(&self, root_path: PathBuf) -> Result<ProjectTree, String>;

    /// XMLファイル (.eeproj) として保存する
    fn save(&self, tree: &ProjectTree) -> Result<(), String>;

    /// 既存の XMLファイル (.eeproj) を読み込む
    fn load(&self, path: PathBuf) -> Result<ProjectTree, String>;
}
