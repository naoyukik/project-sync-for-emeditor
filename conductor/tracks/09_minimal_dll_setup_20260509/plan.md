# Implementation Plan: 最小構成のDLL（器）の実装とEmEditorへのプラグイン認識

## フェーズ 1: 問題の把握と詳細設計 (Discovery & Detailed Design)
- [ ] Task: `autonomous-researcher` スキルによる詳細調査と `evidence_report.md` の作成 (エクスポート関数の正確なシグネチャ、`.def` ファイルの適用方法の調査)
- [ ] Task: 調査結果に基づいた `plan.md` の以降のタスクの具体化
- [ ] Task: Conductor - User Manual Verification 'フェーズ 1: 問題の把握と詳細設計' (Protocol in workflow.md)

## フェーズ 2: ビルド設定の構築
- [ ] Task: `Cargo.toml` とビルドスクリプトの更新
    - [ ] `cdylib` クレートタイプの設定
    - [ ] `windows` クレートの追加 (MessageBoxW, HWND 等)
- [ ] Task: `.def` ファイルの設定
    - [ ] EmEditorプラグインとして必須のエクスポート関数を定義した `exports.def` ファイルの作成。
    - [ ] `build.rs` または `.cargo/config.toml` を用いて `.def` ファイルをリンカに渡す設定を行う。
- [ ] Task: `cargo clippy` の実行と警告の解消
- [ ] Task: `cargo fmt` の実行
- [ ] Task: Conductor - User Manual Verification 'フェーズ 2: ビルド設定の構築' (Protocol in workflow.md)

## フェーズ 3: プラグイン関数の実装と検証
- [ ] Task: エクスポート関数とHelloWorldの実装 (`src/lib.rs` 等)
    - [ ] `DllMain` の実装
    - [ ] 必須エクスポート関数 (`OnCommand`, `PlugInProc`, `QueryStatus` 等) の実装
    - [ ] `OnCommand` 発火時に Win32 API の `MessageBoxW` を呼び出して "HelloWorld" ダイアログを表示する処理の追加。
- [ ] Task: 実機検証
    - [ ] `install.ps1` などを用いてDLLをEmEditorに登録し、メニューから実行して動作を確認する。
- [ ] Task: `cargo clippy` の実行と警告の解消
- [ ] Task: `cargo fmt` の実行
- [ ] Task: Conductor - User Manual Verification 'フェーズ 3: プラグイン関数の実装と検証' (Protocol in workflow.md)