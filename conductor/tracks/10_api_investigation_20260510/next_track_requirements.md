# 次期トラック要件定義: Projectプラグイン自動登録の実装

## 1. 目的
`autonomous-researcher` による調査結果に基づき、`.eeproj` ファイルの直接操作によるプロジェクトツリーの自動更新機能を実装する。

## 2. 主要要件

### 2.1 XML同期エンジン (Rust)
- **フォルダスキャン**: 指定されたルートフォルダ（ワークスペース）を再帰的にスキャンし、現時点のファイルシステム構造を取得する。
- **XMLパース**: 既存の `.eeproj` ファイルを読み込み、メモリ上のツリー構造に変換する。
- **差分比較 (Diffing)**: 
    - ファイルシステムに存在するが XML にない要素を「追加」。
    - XML に存在するがファイルシステムにない要素を「削除」。
- **XML生成**: 差分適用後のツリーを `.eeproj` 形式（`<Project><Files><Filter/File>...`）で書き出す。
- **パス正規化**: すべてのファイルパスは `.eeproj` ファイルからの相対パスとして管理する。

### 2.2 EmEditor DLL 連携
- **リロード命令**: `Editor_ExecutePlugin` API を使用して、`Projects.dll` に対して現在のソリューションを再読み込みさせる。
    - メッセージ: `eePluginUserMessage`
    - パラメータ: ソリューションファイルのフルパス
- **デバウンス処理**: ファイル変更が連続して発生した場合、不必要なリロードを避けるために一定時間（例: 500ms）待機してから実行する。

### 2.3 アーキテクチャ (Strict Rigid Layered Architecture 遵守)
- **Domain**: `ProjectTree` エンティティ、`ProjectRepository` トレイトの定義。
- **Application**: `SyncProjectWorkflow` の実装。
- **Infrastructure**: `XmlProjectRepositoryImpl` (XML操作)、`EmEditorGuiDriver` (ExecutePlugin呼び出し) の実装。

## 3. 制約事項
- **XMLフォーマットの維持**: EmEditorが生成するXMLフォーマットと完全な互換性を保つこと。
- **排他制御**: EmEditor自身がプロジェクトファイルを保存するタイミングとの競合を考慮する（リトライ処理等）。
- **パフォーマンス**: 数千ファイル規模のプロジェクトでも、スキャンとXML生成が高速（100ms以内）に完了すること。

## 4. 次期トラックの構成案
- **Track ID**: `11_implement_xml_sync_20260511`
- **Description**: Projectプラグインへの自動登録機能の実装 (XML操作アプローチ)
