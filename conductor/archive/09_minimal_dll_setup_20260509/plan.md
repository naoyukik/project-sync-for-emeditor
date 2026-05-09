# Implementation Plan: 最小構成のDLL（器）の実装とEmEditorへのプラグイン認識

## フェーズ 1: 問題の把握と詳細設計 (Discovery & Detailed Design)
- [x] Task: `autonomous-researcher` スキルによる詳細調査と `evidence_report.md` の作成 (エクスポート関数の正確なシグネチャ、`.def` ファイルの適用方法の調査)
- [x] Task: 調査結果に基づいた `plan.md` の以降のタスクの具体化
- [x] Task: Conductor - User Manual Verification 'フェーズ 1: 問題の把握と詳細設計' (Protocol in workflow.md)

## フェーズ 2: ビルド設定の構築
- [x] Task: `Cargo.toml` の更新
    - [x] `lib` セクションに `crate-type = ["cdylib"]` を追加
    - [x] `windows` クレートの `Win32_UI_WindowsAndMessaging`, `Win32_Foundation` フィーチャを有効化
- [x] Task: `exports.def` の作成
    - [x] `EXPORTS` 欄に `OnCommand`, `QueryStatus`, `GetMenuTextID`, `GetStatusMessageID`, `GetBitmapID`, `OnEvents`, `PlugInProc` を記述
- [x] Task: `build.rs` の作成
    - [x] `println!("cargo:rustc-cdylib-link-arg=/DEF:exports.def");` を実装
- [x] Task: `cargo clippy` の実行と警告の解消
- [x] Task: `cargo fmt` の実行
- [x] Task: Conductor - User Manual Verification 'フェーズ 2: ビルド設定の構築' (Protocol in workflow.md) [checkpoint: init-kit]

## フェーズ 3: プラグイン関数の実装と検証
- [x] Task: `src/lib.rs` へのエクスポート関数実装
    - [x] `extern "system"` を用いた必須関数のスケルトン実装
    - [x] `OnCommand` 内での `MessageBoxW` (HelloWorld) 実装
- [x] Task: 実機検証
    - [x] `cargo build` の実行
    - [x] 生成されたDLLを EmEditor のプラグインディレクトリへ配置（または直接登録）
    - [x] メニューから実行し、MessageBoxが表示されることを確認
- [x] Task: `cargo clippy` の実行と警告の解消
- [x] Task: `cargo fmt` の実行
- [x] Task: Conductor - User Manual Verification 'フェーズ 3: プラグイン関数の実装と検証' (Protocol in workflow.md) [checkpoint: 2b35700]

## フェーズ 4: レビュー指摘事項の適用 (Review Fixes)
- [x] Task: build.rs のパニックメッセージにエラー詳細を追加
- [x] Task: src/lib.rs に HelloWorld ロジックと Safety Comment を復元
- [x] Task: `cargo clippy` & `cargo fmt` の実行
- [x] Task: Conductor - User Manual Verification 'フェーズ 4: レビュー指摘事項の適用' (Protocol in workflow.md)

## フェーズ 5: 追加レビュー指摘事項の適用 (Final Polish & Review Fixes)
- [x] Task: `src/lib.rs` の全エクスポート関数を `extern "system"` に統一（呼出規約不一致の解消）
- [x] Task: `src/lib.rs` の `OnEvents` シグネチャを SDK (`HWND, u32, LPARAM`) に修正
- [x] Task: `build.rs` を修正し、生成ファイルを `OUT_DIR` 配下へ出力し `include!` で読み込む形式に変更
- [x] Task: `build.rs` の `/DEF:` パスを引用符で囲み、スペースを含むパスでのリンク失敗を防止
- [x] Task: `cargo clippy` & `cargo fmt` の実行
- [ ] Task: 実機検証（MessageBoxの表示が正常か再確認）
- [ ] Task: Conductor - User Manual Verification 'フェーズ 5: 追加レビュー指摘事項の適用' (Protocol in workflow.md)
