# Specification: GitHub Issue 10 - ProjectプラグインAPI調査

## 1. Overview
GitHub Issue 10の解決に向けた事前調査（Investigation）トラック。
EmEditorのProjectプラグインに対して、ファイルやフォルダを動的に追加するための公開APIが存在するかを調査する。
また、APIが存在しなかった場合の代替手段として、Projectプラグインの管理ファイル（XML等）を直接操作・再構築するアプローチの技術的実現性も併せて調査する。

## 2. Investigation Scope (調査スコープ)
- **主目的**: EmEditor SDK、マクロリファレンス、および公式ドキュメントを通じた「Projectプラグイン操作API」の有無の確認。
- **副目的 (代替案)**: APIが存在しなかった場合における、ProjectプラグインのXML構造の解析および外部からの再構築手法の調査。

## 3. Investigation Methodology (調査手法)
- `autonomous-researcher` スキルを用いた、ローカルSDKファイル (`sdk/` 配下) の走査。
- EmEditor公式リファレンスサイトの確認。
- （必要に応じて）Web検索による過去のフォーラムや事例の調査。

## 4. Deliverables (成果物)
- **Evidence Report**: `evidence_report.md` の作成。
  - 発見されたAPI、またはAPIが存在しないことの証拠。
  - XML再構築アプローチの技術的実現性とリスク評価。
  - 今後の実装トラックに向けた「推奨方針」の提示。

## 5. Out of Scope (スコープ外)
- 本トラック内での実際のコード実装や機能改修。
- Projectプラグイン自体の高度なリバースエンジニアリング。
