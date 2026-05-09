# project-sync-for-emeditor - Project Instructions

## Project Overview
EmEditorの既存の「プロジェクトプラグイン」と連携し、手動で行われているフォルダやファイルの登録作業を自動化するプラグインです。バックグラウンドで動作し、指定したフォルダ配下の変更を監視・自動登録することで、ユーザーの開発体験を向上させることを目的としています。

- **Main Technologies:** Rust (Edition 2024), Win32 API, windows-rs
- **Architecture:** Strict Rigid Layered Architecture (物理的隔離による統制)

## Project Layout
- `src/`: Rustのソースコード。
- `conductor/`: プロジェクト管理、設計書、ワークフロー、および進行中のトラック（プラン）。
- `.agents/`: プロジェクト固有のスキル（`SKILL.md`）およびエージェント設定。
- `temporary.local/`: 作業用の一時ファイル（GitHub Issueの本文など）。リポジトリにはコミットしない。

## Building and Running
プロジェクトのビルドと実行には以下のコマンドを使用します。

- **Build:** `cargo build`
- **Test:** `cargo test`
- **Lint:** `cargo clippy`
- **Format:** `cargo fmt`

※ 具体的なビルドターゲットやプラグインのインストール手順は `conductor/workflow.md` または `install.ps1` を参照してください。

## Engineering Conventions
- **Layered Architecture:** `Presentation`, `Application`, `Domain`, `Infrastructure` の4レイヤーに厳密に分離し、依存方向は常に「内側」へ向けてください。
- **Suffix Rule:** ファイル名には役割に応じた接尾辞（`_resolver`, `_workflow`, `_entity`, `_repository_impl` 等）を付けてください。
- **Commit Convention:** コミットメッセージには `feat`, `fix`, `refactor`, `chore` 等のプレフィックスを付け、可能な限りIssue番号（`ref: #xx`）を含めてください。

## Constraints & Safety
- **mod.rs 制限:** `mod.rs` はモジュールの公開範囲と再エクスポートの定義のみに限定し、ロジックを記述しないでください。
- **300行制限:** 1ファイルが300行（実質行数）を超えた場合、即座にモジュール分割を行ってください。
- **Resource Management:** Win32ハンドル（`HWND`等）は必ずRAIIパターン（`Drop`トレイト）を用いて管理し、リークを防止してください。
- **Credential Protection:** `.env` ファイルや秘密鍵を絶対にコミットしないでください。

## Definition of Done (DoD)
作業が「完了」したとみなすための基準です：
1. **実装:** `spec.md` および `plan.md` の全要件を満たしている。
2. **テスト:** `cargo test` がパスし、新しいコードに対して **92%以上** のカバレッジを達成している。
3. **品質ゲート:** `conductor/workflow.md` に定義された全ての品質チェック（型安全、ドキュメント、リンター等）をクリアしている。
4. **検証:** 計画された手動検証手順を実行し、期待通りの動作が確認されている。

詳細は `conductor/index.md` を起点に各ドキュメントを確認してください。

## Git 操作原則 (Git Standards)

履歴の整合性と透明性を保つため、一括操作を避け、厳密なステージングを行うこと。

- **個別指定の徹底**: 変更したファイルは原則として個別に `git add <file>` で指定すること。`git add .` や `git add -A` は禁止。
- **ドットフォルダ**: `.gemini/...` 等もリポジトリ相対パスで個別に指定すること。
- **Conductor 例外**: `conductor/` 配下のみ、整合性確保のため `git add conductor/` を許可する。
- **事前監査**: ステージング前後で `git diff` を実行し、意図しない変更（デバッグ残し等）を排除せよ。

## コミット規約 (Commit Convention)

[Conventional Commits](https://www.conventionalcommits.org/en/) を採用し、以下の形式を維持すること。
**必ず日本語で記述すること。**

### フォーマット
- **1行目: タイトル**: `<type>: 日本語での説明（50文字以内）`
- **空行**
- **説明文(optional)**: 自明な説明はせずに、なぜその変更が必要なのか、もしくは何を達成するための実装なのかを完結に記述すること。箇条書きで記載すること
- **空行**
- **参照**: `ref: IssueNumber` を記述すること。IssueNumberはGitブランチの `^[0-9]+-` にマッチする数字のこと
- **空行**
- **署名**: メッセージ末尾に `Co-Authored-By:` トレイラーを付与すること。キー名は必ず `Co-Authored-By:` のままとし、AIごとに差し替えるのは `名前 <メールアドレス>` の値部分のみとする。

e.g. ブランチ名: 110-implement-font-style-selection
```text
feat: 設定ダイアログにフォントスタイル選択を追加

- ユーザーが好みのフォントスタイルを選択できるにするために実装した 

ref: 110

Co-Authored-By: gemini-cli <218195315+gemini-cli@users.noreply.github.com>
```

## アーキテクチャ原則 (Architecture Principles)

「Strict Rigid レイヤードアーキテクチャ」を遵守せよ。

- **サフィックスによる責務分割**: `_resolver`, `_gui_driver`, `_workflow`, `_entity`, `_repository_impl`, `_io_driver`。
- **依存の方向**: 常に「外側 → 内側（Domain/Application）」へ。
- **API 隔離**: `windows-rs` の型は `_gui_driver` と `_io_driver` にのみ封印すること。
- **詳細**: `conductor/code_styleguides/architecture_rules.md` を参照せよ。

## Done の定義 (Definition of Done)

タスクの完了は、以下の条件をすべて満たした状態を指す。

- [ ] 実装が `Architecture Principles` に準拠している。
- [ ] `cargo clippy` および `cargo fmt` がパスしている。
- [ ] `install.ps1` による実機動作確認が成功し、ユーザーの承認を得ている。
- [ ] 関連する設計ドキュメント（Conductor）が更新されている。
- [ ] 適切なテストコードが追加または更新されている。

## 継続的な改善 (Retro-action)

- 同じ間違いを二度繰り返した場合、直ちに原因を特定し、本 `AGENTS.md` または関連する `styleguide` を更新して再発を防止せよ。
- ユーザーからのフィードバックや、開発中に発見した「より良い手法」は、積極的にプロジェクトの知識ベースへ反映させること。

## コミュニケーションと言語 (Communication)

- **日本語の使用**: 対話、レポート、プラン、コミットメッセージ、および各 Extensions の出力において常に日本語を使用せよ。
- **トーン**: 冷静、知的、かつ厳格な「だ・である」調（常体）を維持せよ。

---

## 開発参考資料
- **EmEditor Plugin SDK**: [公式ドキュメント](https://www.emeditor.com/sdk/)（`sdk/` 内も参照）
- **Learn Microsoft**: `learn-microsoft` ツールにより Windows API を検索可能。

---

## プロジェクト情報
- **リポジトリ**: https://github.com/naoyukik/emeditor-terminal
- **主要言語**: Rust (Win32 API)
- **ログ位置**: `$env:TEMP\emeditor_terminal.log`
