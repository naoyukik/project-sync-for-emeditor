# Product Guidelines

## Core Principles
1. **Invisible Operation (Transparent UX):** プラグインはバックグラウンドで静かに動作しなければならない。不必要なポップアップでユーザーの作業を中断させたり、フォーカスを奪ったりしてはならない。
2. **Performance First:** エディタの拡張機能として、タイピングの応答性を低下させたり、UIをフリーズさせたりすることは絶対に避ける。ファイルシステムの監視とプロジェクトツリーの更新は必ず非同期で行うこと。
3. **Graceful Degradation:** エラー（権限エラーやディスクのアンマウント等）が発生した場合でも、EmEditor本体を巻き込んでクラッシュすることなく、安全に失敗しなければならない。

## UX Guidelines
- **Zero Configuration Default:** 特別な設定を行わずとも、有効化された時点ですぐに機能し始めること（例: 現在開いているフォルダを自動的に監視対象とするなど、直感的な初期動作）。
- **Silent Logging:** 動作ログや軽微なエラーはバックグラウンドのログファイルに記録し、致命的なエラーでない限りメッセージボックスを表示しないこと。

## Technical Directives
- **Resource Management:** 監視対象のファイル数が膨大になった場合でも、メモリリークや過度なCPU使用を避けるため、RustのRAIIパターンを用いた厳密なリソース管理を行う。