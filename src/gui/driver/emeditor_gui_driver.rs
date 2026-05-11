use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, WM_USER};

/// EmEditor固有のGUI操作を担当するドライバー
pub struct EmEditorGuiDriver {
    hwnd: HWND,
}

impl EmEditorGuiDriver {
    pub fn new(hwnd: HWND) -> Self {
        Self { hwnd }
    }

    /// Projects.dll に対してプロジェクトの更新を指示する
    pub fn reload_projects_plugin(&self) -> Result<(), String> {
        // EE_EXECUTE_PLUGIN: (WM_USER + 191)
        const EE_EXECUTE_PLUGIN: u32 = WM_USER + 191;

        // Projects.dll のリロードコマンド (仮定: 1)
        // 実際には Editor_ExecutePlugin を介して呼ばれる
        // windows-rs では SendMessage を用いて EmEditor メインウィンドウに送信する

        unsafe {
            // wParam: "Projects.dll" へのポインタ
            // lParam: コマンドID (1: リフレッシュ)
            // 注意: 本来は Editor_ExecutePlugin インライン関数を使用するのが安全だが、
            // Rustからは直接 SendMessage を用いる。

            // TODO: 文字列の渡し方など、詳細なSDK仕様に合わせた実装
            // 現時点では、SendMessageW を用いた骨組みのみ作成
            SendMessageW(
                self.hwnd,
                EE_EXECUTE_PLUGIN,
                None, // "Projects.dll" を渡す必要がある
                None, // コマンド ID を渡す必要がある
            );
        }

        Ok(())
    }
}
