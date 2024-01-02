use tauri::{App, Manager, WindowBuilder};
use window_vibrancy::apply_blur;

/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_window("main").unwrap();
    win.show().unwrap();
    win.set_decorations(true).unwrap();

    use window_vibrancy::{apply_acrylic, apply_vibrancy, NSVisualEffectMaterial};
    use window_shadows::set_shadow;
    use window_vibrancy::NSVisualEffectState;

    #[cfg(target_os = "macos")]
    {
    apply_vibrancy(&win, NSVisualEffectMaterial::HudWindow, Some(NSVisualEffectState::Active), None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
    }

    #[cfg(target_os = "windows")]
    apply_acrylic(&win, Some((255, 241, 235, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    set_shadow(&win, true).unwrap();
    Ok(())
}