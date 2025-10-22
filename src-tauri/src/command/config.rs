use tauri::{Manager, Runtime};

#[tauri::command]
pub async fn put_config<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    let a = app.cursor_position().unwrap();
    let b = window.inner_position().unwrap();
    let c = app.path().app_data_dir().unwrap();
    let e = app.path().local_data_dir().unwrap();
    let f = app.path().app_config_dir().unwrap();

    app.remove_menu().unwrap();

    println!(
        "{}, {}, {}, {}, {}, {}, {}",
        a.x,
        a.y,
        b.x,
        b.y,
        c.display(),
        e.display(),
        f.display(),
    );

    Ok(())
}
