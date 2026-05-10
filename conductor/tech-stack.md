# Tech Stack

## プログラミング言語
- **Rust:** ファイルシステムの監視やI/O処理を含むコアロジックはRustで実装する。メモリ安全性、高いパフォーマンス、システムレベルのプログラミングに対する強固なエコシステムを理由に選定している。

## 基盤ライブラリ
- **windows (v0.62.2):** EmEditor SDK および Win32 API 連携用。
- **notify (v8.0):** ファイルシステム監視用。

## ビルド構成
- **DLL (cdylib):** EmEditorプラグインとして動作するため、ダイナミックリンクライブラリ形式でビルドされる。
- **Linker Control:** `exports.def` ファイルと `build.rs` (`cargo:rustc-cdylib-link-arg=/DEF:...`) を使用し、Manglingを避けた正確なシンボルエクスポートを制御。
- **Resource Handling:** `build.rs` で `resource.h` から Rust 定数を自動生成し、`.rc` ファイルを埋め込む。

## データベース
- **なし:** 本プラグインはローカルのファイルシステム上で直接動作し、EmEditorのプロジェクトファイルやAPIと連携するため、外部データベースは使用しない。