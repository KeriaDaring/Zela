use tauri::{App, Manager, WindowBuilder};
use window_vibrancy::apply_blur;

/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_window("main").unwrap();
    win.show().unwrap();


    use window_vibrancy::{apply_acrylic, apply_vibrancy, NSVisualEffectMaterial};
    use window_shadows::set_shadow;
    use window_vibrancy::NSVisualEffectState;

    #[cfg(target_os = "macos")]
    {
        win.set_decorations(true).unwrap();
        win.set_titlebar_style("Overlay").unwrap();
        apply_vibrancy(&win, NSVisualEffectMaterial::HudWindow, Some(NSVisualEffectState::Active), None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");


    }

    #[cfg(target_os = "windows")]
    {
        apply_acrylic(&win, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
        win.set_decorations(true).unwrap();
    }


    Ok(())
}