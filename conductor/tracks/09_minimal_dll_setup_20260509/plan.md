# Implementation Plan: 最小構成のDLL（器）の実装とEmEditorへのプラグイン認識

## フェーズ 1: 問題の把握と詳細設計 (Discovery & Detailed Design)
- [x] Task: `autonomous-researcher` スキルによる詳細調査と `evidence_report.md` の作成 (エクスポート関数の正確なシグネチャ、`.def` ファイルの適用方法の調査)
- [x] Task: 調査結果に基づいた `plan.md` の以降のタスクの具体化
- [~] Task: Conductor - User Manual Verification 'フェーズ 1: 問題の把握と詳細設計' (Protocol in workflow.md)

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
- [ ] Task: `src/lib.rs` へのエクスポート関数実装
    - [ ] `extern "system"` を用いた必須関数のスケルトン実装
    - [ ] `OnCommand` 内での `MessageBoxW` (HelloWorld) 実装
- [ ] Task: 実機検証
    - [ ] `cargo build` の実行
    - [ ] 生成されたDLLを EmEditor のプラグインディレクトリへ配置（または直接登録）
    - [ ] メニューから実行し、MessageBoxが表示されることを確認
- [ ] Task: `cargo clippy` の実行と警告の解消
- [ ] Task: `cargo fmt` の実行
- [ ] Task: Conductor - User Manual Verification 'フェーズ 3: プラグイン関数の実装と検証' (Protocol in workflow.md)