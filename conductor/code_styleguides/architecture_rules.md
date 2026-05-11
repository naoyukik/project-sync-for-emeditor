# **Rust/Win32 レイヤードアーキテクチャの掟 (Strict Rigid版)**

AIエージェントに「ノリ」でコードを書かせず、物理的なファイル名と配置によって役割と境界を統制するための、最高レベルに厳格な設計ガイドである。

## **1. コンセプト：物理的隔離による統制 (The Suffix Rule)**

ファイル名に特定の接尾辞（サフィックス）を義務付け、そのファイルが「知ってよい情報」と「依存してよい方向」を物理的に固定する。
また、**サフィックスごとに配置すべきディレクトリが厳格に定められている。**

| レイヤー | 接尾辞 | 配置ディレクトリ (`src/`配下) | 役割と指示の掟 |
| :---- | :---- | :---- | :---- |
| **1. Presentation** | `_resolver.rs` | `gui/resolver` | OSメッセージ（WndProc等）の解釈。Application層へのディスパッチ。 |
| **1.1. GUI Driver** | `_gui_driver.rs` | `gui/driver` | **描画・IME等のWin32操作**。最外周の「手足」。 |
| **1.5. DTO (GUI用)** | `_request.rs` / `_response.rs` | `gui` | Presentation層専用。Windowsメッセージ構造と1:1で対応。 |
| **2. Application** | `_workflow.rs` | `application` | 処理のシナリオ（ユースケース）。DTOとEntityを変換する。 |
| **2.5. DTO (App用)** | `_input.rs` / `_result.rs` | `application` | Application層の境界型。Entityを直接外部に漏らさない。 |
| **3. Domain** | `_entity.rs` / `_value.rs` | `domain/model` | アプリの核心。**外部DTOや外部プロトコルTraitを一切参照してはならない。** |
| **3.1. Domain Service** | `_domain_service.rs` | `domain/service` | Entityに収まらないドメインロジック。 |
| **3.2. Protocol Handler** | `_protocol_handler.rs` | `domain/service` | **外部プロトコルの解釈とEntityへの操作命令**。Entityと外部ライブラリの絶縁層。 |
| **3.5. Repository (IF)** | `_repository.rs` | `domain/repository` | **Trait定義**。Domain層に属し、外部へのデータ要求を定義する。 |
| **4. Infrastructure** | `_repository_impl.rs` | `infra/repository` | Repository Traitの具象実装。IO Driverを使用する。 |
| **4.1. IO Driver** | `_io_driver.rs` | `infra/driver` | **外部操作（File/SDK等）**。最外周の「手足」。 |

**例外規定 (Whitelist)**: 
* `mod.rs`, `lib.rs`, `main.rs`, `build.rs`: モジュール管理およびエントリーポイント。
* `resource.rs`: `resource.h` から `build.rs` によって自動生成される Win32 リソース定数定義ファイル。

## **2. Windows API (windows-rs) の隔離命令**

Windows APIの型は汚染力が強いため、以下の隔離を徹底せよ。

* **`_gui_driver.rs` / `_io_driver.rs`**: `windows` クレートの型（`HWND`, `RECT`, `COLORREF`等）を扱ってよい唯一の場所。
* **`_resolver.rs`**: Win32型を即座に内部型（Input DTO等）へ変換せよ。
* **重罪**: `Domain` 層および `Application` 層において、`windows` クレートを直接 `use` することを固く禁ずる。必要な定数（VKコード等）はDomain層でPure Rust定義として re-define せよ。

## **3. プロトコル解析 (vte等) の隔離命令**

ターミナル・シーケンス（ANSI/VT）の解釈ロジックは、外部仕様（プロトコル）と内部データ（Entity）の境界を跨ぐため、特に混同しやすい。以下の隔離を徹底せよ。

* **`_protocol_handler.rs`**: `vte::Perform` 等の外部ライブラリのトレイトを実装してよい唯一の場所。ここが Entity の操作メソッド（`move_cursor`, `put_char` 等）を呼び出す「指揮官」となる。
* **`_entity.rs`**: プロトコル解析用の巨大な `match` 文（`csi_dispatch` 等）をここに記述することを固く禁ずる。Entity は「言われた通りの座標に文字を置く」といった、純粋なバッファ操作に専念せよ。
* **利点**: 将来、別のパーサーライブラリに変更する場合や、テスト用のダミーパースを行う場合に、Entity 側を一切変更せずに済むようになる。

## **4. ファイル分離と行数制限の掟**

AIエージェントのコンテキスト理解を助け、編集の正確性を高めるために以下の分離を徹底せよ。

* **1ファイル1責務**: `DTO`, `Input`, `Result`, `Entity` はすべて別ファイルに切り出すこと。
* **300行制限の義務化**: 1ファイルが300行（コメント等を除く実質行数）を超えた場合、設計に瑕疵がある、あるいは責務が過剰であると判断し、即座にモジュール分割を行うこと。特に `_entity.rs` の肥大化は「神クラス」への予兆であり、絶対に許容しない。
* **mod.rs の役割制限**: `mod.rs` はモジュールの公開範囲（`pub mod`）と再エクスポート（`pub use`）の定義のみに限定せよ。ロジック（関数、構造体、impl ブロック等）の記述を固く禁ずる。
* **AI最適化**: ファイルを小さく保ち、シンボルジャンプに頼らずともファイル構成だけで依存関係を理解できるようにせよ。

## **5. 依存方向とデータフローの絶対則**

依存の矢印は常に「内側（Domain/Application）」へ向かい、Win32 APIは両端の「Driver」に封印する。
また、**具象実装 (`infra` / `impl`) への直接依存は厳禁であり、必ずインターフェース (Repository Trait) を経由せよ。**

### **レイヤー別依存許可リスト**

| 参照元レイヤー (`src/`配下) | 許可される依存先パス |
| :--- | :--- |
| `gui/resolver` | `application`, `domain`, `gui/driver` |
| `gui/driver` | `domain` |
| `application` | `domain` |
| `domain` | (なし。完全な独立) |
| `infra/repository` | `domain`, `infra/driver` |
| `infra/driver` | `domain` |

※ `common` 等の共通ユーティリティへの依存は全レイヤーで許可される。

### **データフロー図**

## **5. ユビキタス言語辞典 (Ubiquitous Language Dictionary)**

特定の文脈において、以下の命名を「正しいドメイン用語」として強制する。

| 概念 | 推奨される名称 | 除外すべき汎用名 |
| :--- | :--- | :--- |
| ウィンドウ共有データ | `window_data` | `data` |
| 入力用バイト列 | `input_bytes` | `data`, `buf` |
| 受信テキスト | `output_text` | `output` |
| 文字属性（太字等） | `is_bold`, `is_italic` 等 | `bold`, `italic` (形容詞単体) |
| 可視性状態 | `is_visible` | `visible` |
| キー押下状態 | `is_ctrl_pressed` 等 | `ctrl`, `shift` (単一名) |
| スクロール位置 | `viewport_offset`, `scroll_pos` | `pos`, `offset` |
| 履歴・バックバッファ | `history`, `back_buffer` | `old_data`, `cache` |
