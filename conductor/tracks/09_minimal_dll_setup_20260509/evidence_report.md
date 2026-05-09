# Evidence Report: 最小構成のDLL（器）の実装とEmEditorへのプラグイン認識

## 1. Discovery Summary
本調査では、RustでビルドしたDLLをEmEditorプラグインとして認識させるための要件を特定した。

### 課題
- EmEditorが期待する正確な関数シグネチャの特定。
- Rust (`cdylib`) ビルドにおいて、リンカに `.def` ファイルを渡し、正しい名前で関数をエクスポートする方法。

### 成功条件
- 全ての必須関数が `extern "system"` で正しく定義されている。
- `exports.def` がビルドプロセスに組み込まれている。
- `OnCommand` 時に `MessageBoxW` が表示される。

## 2. Codebase Findings
- `sdk/hello_plugin/` 内の `etlframe.h` および `hello.cpp` を調査し、以下のシグネチャを確認した。

### 関数シグネチャ (C++ vs Rust)
| 関数名 | C++ シグネチャ | Rust シグネチャ案 |
| :--- | :--- | :--- |
| `OnCommand` | `void __stdcall (HWND)` | `pub extern "system" fn (HWND)` |
| `QueryStatus` | `BOOL __stdcall (HWND, LPBOOL)` | `pub extern "system" fn (HWND, *mut BOOL) -> BOOL` |
| `GetMenuTextID` | `UINT __stdcall ()` | `pub extern "system" fn () -> u32` |
| `GetStatusMessageID` | `UINT __stdcall ()` | `pub extern "system" fn () -> u32` |
| `GetBitmapID` | `UINT __stdcall ()` | `pub extern "system" fn () -> u32` |
| `OnEvents` | `void __stdcall (HWND, UINT, LPARAM)` | `pub extern "system" fn (HWND, u32, LPARAM)` |
| `PlugInProc` | `LRESULT __stdcall (HWND, UINT, WPARAM, LPARAM)` | `pub extern "system" fn (HWND, u32, WPARAM, LPARAM) -> LRESULT` |

## 3. Evidence
- **EmEditor SDK**: `sdk/hello_plugin/etlframe.h` 764行目付近。
- **Rust .def指定**: `build.rs` 内で `println!("cargo:rustc-cdylib-link-arg=/DEF:exports.def");` を使用する手法がMSVC環境において標準的である。

## 4. Architecture Options

### Option A: Minimal Monolith (Recommended for this track)
- `src/lib.rs` に全てのエクスポート関数を記述。
- `OnCommand` 内部で直接 `MessageBoxW` を呼ぶ。
- メリット: 構造が単純で、ビルドと認識の確認に集中できる。

### Option B: Layered Dispatcher
- `src/lib.rs` はエントリポイントのみ。
- `src/gui/driver/terminal_gui_driver.rs` (仮) に実際のUI処理（MessageBox）を委譲。
- メリット: 本プロジェクトの「Strict Rigid レイヤードアーキテクチャ」に忠実。
- 推奨: 今回は「器」の構築が目的であるため、Option Aをベースにしつつ、将来的にOption Bへ移行しやすいよう、`lib.rs` を薄く保つ。

## 5. Implementation Details
- `Cargo.toml`: `crate-type = ["cdylib"]` を追加。
- `exports.def`: プロジェクトルートに作成し、関数名を列挙。
- `build.rs`: リンカ引数の追加処理を記述。
