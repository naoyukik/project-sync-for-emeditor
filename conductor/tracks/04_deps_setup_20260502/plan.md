# Implementation Plan: プラグイン作成に必要なライブラリの調査と追加 (Issue #4)

## フェーズ 1: 問題の把握と詳細設計 (Discovery & Detailed Design)
- [x] Task: `autonomous-researcher` スキルによる詳細調査と `evidence_report.md` の作成
- [x] Task: 調査結果に基づいた `plan.md` の以降のタスクの具体化
- [~] Task: Conductor - User Manual Verification 'フェーズ 1: 問題の把握と詳細設計' (Protocol in workflow.md)

## フェーズ 2: 依存関係の追加とダミーエントリポイント実装 (Specific Implementation)
- [ ] Task: `Cargo.toml` の設定変更とライブラリ追加
    - [ ] `[lib]` セクションに `crate-type = ["cdylib"]` を追加し、名前を `project_sync_for_emeditor` に設定。
    - [ ] `windows` クレートを追加。フィーチャー: `Win32_Foundation`, `Win32_System_SystemServices`, `Win32_UI_WindowsAndMessaging`
    - [ ] `notify` クレートを追加。
- [ ] Task: `DllMain` 等のダミーエントリポイントの実装
    - [ ] `src/lib.rs` を作成。
    - [ ] `extern "system" fn DllMain` を実装し、`#[no_mangle]` を付与。
    - [ ] モジュールハンドルを保存するためのグローバル変数（`static mut G_HINSTANCE`）を定義。
- [ ] Task: `cargo clippy` の実行と警告の解消
- [ ] Task: `cargo fmt` の実行
- [ ] Task: Conductor - User Manual Verification 'フェーズ 2: 依存関係の追加とダミーエントリポイント実装' (Protocol in workflow.md)