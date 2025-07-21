use crate::emit::{repo_single_emit, satatus_change_emit};
use crate::utils::init;
use crate::{core::handle, emit::changed_emit};
use anyhow::Result;
use giter_watcher::modify_watcher::ModifyWatcher;
use std::sync::Mutex;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, Manager};

pub async fn resolve_setup(app: &mut App) -> Result<()> {
    handle::Handle::global().init(app.app_handle());
    // 初始化日志
    init::init_log()?;
    println!("init log success");
    init::init_store()?;
    println!("init store success");
    init::init_config()?;
    println!("init conifg success");
    let mut watcher_center = ModifyWatcher::new();
    watcher_center.add_callback(satatus_change_emit);
    watcher_center.add_callback(changed_emit);
    watcher_center.add_callback(repo_single_emit);
    let _ = watcher_center.init();
    // 修改监控器
    app.manage(Mutex::new(watcher_center));

    Ok(())
}

pub async fn init_tray(app: &mut App) -> Result<()> {
    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("Giter")
        .show_menu_on_left_click(true)
        .build(app)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;
    tray.set_menu(Some(menu))?;

    tray.on_menu_event(|app, event| match event.id.as_ref() {
        "quit" => {
            println!("quit menu item was clicked");
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    });

    tray.on_tray_icon_event(|tray, event| match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            let app = tray.app_handle();
            let main_window = app.get_window("main").unwrap();
            if main_window.is_visible().unwrap() {
                main_window.hide().unwrap();
            } else {
                main_window.show().unwrap();
            }
        }
        _ => {}
    });

    let main_window = app.get_window("main").unwrap();
    let main_window_handle = main_window.clone();
    main_window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            main_window_handle.hide().unwrap();
        }
    });
    Ok(())
}
