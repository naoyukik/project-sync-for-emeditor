# Evidence Report: プラグイン作成に必要なライブラリの調査と追加 (Issue #4)

## 調査概要
EmEditorのプロジェクトプラグインと連携するバックグラウンド監視プラグインをRustで実装するための基盤調査を実施した。

## 調査結果 (Discovery & Findings)

### 1. プロジェクト構成 (Cargo.toml)
- **Crate Type**: DLLとしてビルドするため、`[lib]` セクションに `crate-type = ["cdylib"]` が必須。
- **windows-rs**: EmEditor SDKおよびWin32 APIとの連携に使用。
  - 必要なフィーチャー例: `Win32_Foundation`, `Win32_System_SystemServices`, `Win32_UI_WindowsAndMessaging`
- **notify**: ファイルシステム監視に使用。最新バージョン (v8.0) を推奨。

### 2. DllMain の実装要件
- **シグネチャ**: `extern "system" fn DllMain(hinst_dll: HMODULE, fdw_reason: u32, lpv_reserved: *const std::ffi::c_void) -> BOOL`
- **属性**: `#[no_mangle]` が必須。
- **注意点**: 
  - Loader Lockの影響を受けるため、`DllMain` 内での複雑な処理（I/Oや同期）は避け、最小限の初期化（モジュールハンドルの保存等）にとどめる。
  - 重い処理を行う場合はスレッドを生成して即座にリターンする。

### 3. EmEditorプラグインのエントリポイント
- **OnCommand**: ツールバーやメニューから呼ばれる基本関数。
- **呼び出し規約**: `extern "C"`。

### 4. ファイル監視 (notify)
- `RecommendedWatcher` を使用することで、Windows環境では `ReadDirectoryChangesW` が自動的に選択される。
- 監視を継続するには、`watcher` オブジェクトをドロップせずに保持し続ける必要がある。

## 期待される挙動 (Expected Behavior)
- `cargo build` を実行すると、`target/debug/project_sync_for_emeditor.dll` が生成される。
- `DllMain` が実装されており、DLLのロード時にモジュールハンドルが正しく保持される。
- `Cargo.toml` に `windows` と `notify` が定義されており、今後の実装でこれらを利用できる。

## 具体的な修正箇所
- `Cargo.toml`: `[lib]` セクションの追加、`dependencies` へのクレート追加。
- `src/lib.rs`: `DllMain` の実装。

## エビデンス資料
- [windows-rs Documentation](https://microsoft.github.io/windows-docs-rs/)
- [notify crate Documentation](https://docs.rs/notify/latest/notify/)
