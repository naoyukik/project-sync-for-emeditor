# Specification: 最小構成のDLL（器）の実装とEmEditorへのプラグイン認識

## 1. Overview
本格的なロジック実装に先立ち、RustでビルドしたDLLをEmEditorにプラグインとして認識させる「器」を構築する。
これにより、実機での動作確認を早期に可能にし、今後の機能追加の基盤とする。

## 2. Functional Requirements
- **DLL構築**: Rust (`cargo`) を使用して、`cdylib` 形式のDLLをビルドできること。
- **ターゲットアーキテクチャ**: 64-bit (x64) 用のプラグインとしてビルドされること。
- **エクスポート関数**: EmEditorプラグインとして必須となる以下の関数をエクスポートすること。
  - `DllMain`
  - `OnCommand`
  - `QueryStatus`
  - `OnEvents`
  - `GetMenuTextID`
  - `GetStatusMessageID`
  - `GetBitmapID`
  - `PlugInProc`
- **シンボルエクスポート**: `.def` ファイルを用いて、EmEditorが要求する正しい名前で関数がエクスポートされること。
- **HelloWorldコマンド**: プラグインが認識され、メニューから実行された際、Win32 APIの `MessageBoxW` を使用して「HelloWorld」ダイアログを表示すること。

## 3. Non-Functional Requirements
- `windows-rs` クレートを使用してWin32 APIを呼び出す。
- 本プロジェクトの「Strict Rigid レイヤードアーキテクチャ」の原則に従うこと（最小構成ではあるが、後続の実装を見据えたファイル分割を意識する）。

## 4. Acceptance Criteria
- [ ] `cargo build` でエラーなく `.dll` ファイルが生成される。
- [ ] 生成された `.dll` をEmEditorのプラグインとして登録した際、EmEditorのプラグイン一覧に正常に表示される。
- [ ] EmEditorからプラグインを実行した際、エラーでクラッシュすることなく「HelloWorld」のMessageBoxが表示される。

## 5. Out of Scope
- 本質的なファイル検知ロジックやXML生成ロジックの実装（別Issueにて対応）。
- 複雑なダイアログやUIコンポーネントの実装。