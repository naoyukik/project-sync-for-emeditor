use crate::domain::model::project_tree_entity::ProjectTreeEntity;
use std::path::PathBuf;

/// プロジェクトの永続化とスキャンのためのインターフェース
pub trait ProjectRepository {
    /// 現在のファイルシステムの状態をスキャンして ProjectTreeEntity を生成する
    fn scan(&self, root_path: PathBuf) -> Result<ProjectTreeEntity, String>;

    /// XMLファイル (.eeproj) として保存する
    fn save(&self, tree: &ProjectTreeEntity) -> Result<(), String>;

    /// 既存の XMLファイル (.eeproj) を読み込む
    fn load(&self, path: PathBuf) -> Result<ProjectTreeEntity, String>;
}
