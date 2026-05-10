# Specification: Projectプラグイン XML同期エンジンの実装 (Issue 10)

## 1. Overview
EmEditorのProjectプラグイン (`.eeproj`) に対して、指定されたフォルダツリーの構造を反映させるためのコアエンジンを実装する。本トラックではバックグラウンド監視を含まず、ファイルシステムのスキャンからXMLの生成、およびEmEditorへのリロード指示までの一連の処理を構築する。

## 2. Functional Requirements (機能要件)
- **フォルダスキャン**: 指定されたルートディレクトリを再帰的にスキャンし、現在のファイル・フォルダ構造を取得する。
- **XMLパース・生成**: `quick-xml` クレートを使用し、既存の `.eeproj` ファイルの読み込み、差分比較、および新しい `.eeproj` の生成を行う。
- **パスの正規化**: 生成されるXML内の `RelativePath` は、すべて `.eeproj` ファイルからの相対パスとして計算・記述されること。
- **フィルタリング**: `.git`, `target`, `node_modules` などの一般的な管理用フォルダ、およびビルド成果物は自動的にスキャン対象から除外されること。
- **DLL連携 (リロード)**: XMLの更新後、EmEditor SDK (`Editor_ExecutePlugin`) を介して `Projects.dll` に再読み込みを指示する。
- **デバウンス制御**: 連続した同期リクエストが来た場合、一定時間（例: 500ms）待機し、最後の1回だけリロードを実行することでUXの低下を防ぐ。

## 3. Non-Functional Requirements (非機能要件)
- **パフォーマンス**: 数千ファイル規模のプロジェクトであっても、スキャンからXML生成までが十分に高速（100ms以内）に完了すること。
- **アーキテクチャ**: AcePilotの Strict Rigid Layered Architecture (GUI -> App -> Domain <- Infra) を遵守すること。
  - XML操作は Infrastructure レイヤーにカプセル化する。
  - Win32 API (`ExecutePlugin`) 呼び出しは Infrastructure/GUI レイヤーにカプセル化する。

## 4. Acceptance Criteria (受け入れ条件)
- [ ] テストデータを含むフォルダに対して同期処理を実行した際、正しく `.eeproj` 形式のXMLが生成されること。
- [ ] 既存の `.eeproj` が存在する場合、既存の構成（開閉状態など、維持可能なものがあれば）を可能な限り破壊せずに差分更新されること。
- [ ] 同期処理後、EmEditorのプラグインリロード処理が正常に呼び出されること（モックテスト等での確認）。
- [ ] 規定の除外リスト（`.git` 等）配下のファイルがXMLに含まれないこと。

## 5. Out of Scope (スコープ外)
- **バックグラウンド監視**: `notify` クレート等を用いたファイルシステムのリアルタイム監視機能（これは次期以降の別トラックで実装する）。
- EmEditor上の新しいUI要素の追加。