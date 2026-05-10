# Evidence Report: ProjectプラグインAPI調査 (Issue 10)

## 1. Discovery Summary (Phase 1)

- **[Problem Statement]**: EmEditorのProjectプラグインにおいて、ファイルシステムの変更（追加・削除・移動）を検知した際に、手動操作なしでプロジェクトツリーに動的に反映させたい。そのための操作APIの有無、および代替案の実現性を調査する。
- **[Scope]**: 
    - EmEditor SDK (`plugin.h`, `SwitchCustomBar.jsee`等) の調査。
    - 外部リソース（公式フォーラム、コミュニティ）の調査。
    - プロジェクト管理ファイル (`.eesln`, `.eeproj`) の構造解析。
- **[Success Criteria]**: 
    - 信頼できる操作方法（APIまたは代替手段）の特定。
    - 実装フェーズに向けたアーキテクチャ案の提示。

## 2. Codebase Findings (Phase 2)

- **[SDK Definition]**:
    - `sdk\plugin-library\plugin.h` (L6052-6131): Projectプラグイン向けのメッセージが定義されているが、`OPEN_SOLUTION`, `CREATE_TAGS`, `GET_SYMBOLS` 等に限定されており、**個別のファイル/フォルダを追加するメッセージは存在しない**。
    - `sdk\library\macros\SwitchCustomBar.jsee`: プラグインの表示/非表示を切り替える汎用的な `ExecuteCommandByID` の使用例のみ。
- **[XML Structure]**:
    - `.eesln` (Solution): プロジェクトファイル (`.eeproj`) への相対パスとグローバル設定を保持。
    - `.eeproj` (Project): `<Files>` タグ配下に `<Filter Name="...">` (フォルダ) と `<File RelativePath="...">` (ファイル) を持つ。非常にシンプルなXML構造。

## 3. Clarifying Questions (Phase 3)

- **[Open Questions]**:
    1. **Q**: `.eeproj` を直接編集してディスクに保存しただけで反映されるか？
    2. **Q**: `ExecutePlugin` で再読み込みした際、現在のツリーの展開状態や選択位置は維持されるか？
- **[User Answers / Delegations]**:
    - (調査結果に基づく推論) A1: 反映されない可能性が高いため、`ExecutePlugin` による明示的な再読み込みが必要。
    - (調査結果に基づく推論) A2: 公式フォーラム等の情報では、ソリューションの再オープンはツリーの状態をリセットする可能性がある。UXへの影響を最小限にするための工夫（変更がある場合のみ実行する等）が必要。

## 4. 将来の修正で期待される挙動 (Expected Behavior)

- 監視対象フォルダ配下に新しいファイルが作成されると、`.eeproj` の適切な `<Filter>` 配下に `<File>` タグが追加される。
- 既存のプロジェクト構造を壊すことなく、差分のみを正確にXMLに反映する。
- EmEditor上でプロジェクトツリーが最新の状態に更新される。

## 5. Architecture Options (Phase 4)

### Option A: XML Direct Manipulation (推奨)
- **[Change Targets]**: `.eeproj` (XML直接編集), `ExecutePlugin` (再読み込み指示)
- **[Pros]**: 確実に動作する。公開APIの制限を回避できる。
- **[Cons/Risks]**: ファイルI/Oのオーバーヘッド、再読み込み時のUX（ツリーの展開状態等）への影響。
- **[Validation Plan]**: 手動でXMLを書き換え、`ExecutePlugin` マクロを実行して反映を確認する。

### Option B: UI Automation (非推奨)
- **[Change Targets]**: ウィンドウメッセージ送信 (`WM_COMMAND` 等)
- **[Pros]**: XMLを触らなくて済む。
- **[Cons/Risks]**: 非公開コマンドIDへの依存、UIの不整合、極めて不安定。

- **[Recommended Option]**: Option A
- **[Reason]**: 公開APIが存在しない現状において、最も確実かつ保守可能な手法である。XML構造がシンプルであるため、Rust側での生成・編集も容易。

## 6. 推奨される実装方針 (Implementation Strategy)

1. **XML Parser/Generator の導入**: Rustの `quick-xml` や `serde-xml-rs` を検討（軽量さ重視）。
2. **差分更新ロジック**: ファイルシステムの状態と現在の `.eeproj` の内容を比較し、不足している要素を追加、削除された要素を除去する。
3. **リロードトリガー**: `Editor_ExecutePlugin` (またはマクロ経由) を呼び出し、現在のソリューションを再オープンさせる。
4. **UXの考慮**: 更新頻度を制御（デバウンス等）し、不必要な再読み込みを避ける。

## 7. Evidence and Alignment
- EmEditor 公式フォーラム情報: "特定のファイルを追加するAPIは公開されていない"
- SDK 調査結果: `plugin.h` に該当メッセージなし。
- 実機解析: `.eeproj` は標準的なXMLフォーマット。
