# Implementation Plan: Projectプラグイン XML同期エンジンの実装 (Issue 10)

## フェーズ 1: ドメインおよびアプリケーションレイヤーの実装 (Domain & Application)
- [x] Task: `ProjectTree` エンティティおよび関連するデータ構造の実装
    - [x] Sub-task: (Red) `ProjectTree` にファイル・フォルダを追加/削除する振る舞いの単体テストを作成する。
    - [x] Sub-task: (Green) テストをパスする最小限の実装を行う。
- [x] Task: `ProjectRepository` トレイトおよび `SyncProjectWorkflow` (ユースケース) の定義
    - [x] Sub-task: (Red) モックを用いた `SyncProjectWorkflow` の同期ロジックの単体テストを作成する（差分抽出のロジック等）。
    - [x] Sub-task: (Green) テストをパスする最小限の実装を行う。
- [x] Task: `cargo clippy` の実行と警告の解消
- [x] Task: `cargo fmt` の実行
- [x] Task: Conductor - User Manual Verification 'フェーズ 1: ドメインおよびアプリケーションレイヤーの実装' [checkpoint: 0b45a2b] (Protocol in workflow.md)

## フェーズ 2: インフラストラクチャレイヤーの実装 - XML操作 (XML Infrastructure)
- [~] Task: `quick-xml` を用いた `XmlProjectRepositoryImpl` の実装
    - [x] Sub-task: (Red) `.eeproj` フォーマットでのXML生成およびパース処理の単体テストを作成する。
    - [x] Sub-task: (Green) `quick-xml` を用いて、テストをパスするXML生成・パースロジックを実装する。
    - [x] Sub-task: (Red) ファイルシステムのスキャンおよび除外フィルタリング（`.git`等）の単体テストを作成する。
    - [x] Sub-task: (Green) スキャンロジックを実装し、相対パスへの変換処理を含める。
- [x] Task: `cargo clippy` の実行と警告の解消
- [x] Task: `cargo fmt` の実行
- [x] Task: Conductor - User Manual Verification 'フェーズ 2: インフラストラクチャレイヤーの実装 - XML操作' [checkpoint: 62f502b] (Protocol in workflow.md)

## フェーズ 3: インフラストラクチャレイヤーの実装 - DLL連携とデバウンス (DLL Integration)
- [ ] Task: `EmEditorGuiDriver` (または相当するインフラ層) における `ExecutePlugin` の呼び出し実装
    - [ ] Sub-task: `windows` クレートを用いて `Projects.dll` にメッセージを送信する処理を実装する。
- [ ] Task: 同期リクエストのデバウンス制御の実装
    - [ ] Sub-task: (Red) 連続した呼び出しが一定時間（500ms）待機後に1回だけ処理されることの単体テストを作成する。
    - [ ] Sub-task: (Green) デバウンス制御ロジックを実装する。
- [ ] Task: `cargo clippy` の実行と警告の解消
- [ ] Task: `cargo fmt` の実行
- [ ] Task: Conductor - User Manual Verification 'フェーズ 3: インフラストラクチャレイヤーの実装 - DLL連携とデバウンス' (Protocol in workflow.md)