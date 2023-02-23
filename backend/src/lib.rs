#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::sync::Mutex;

use bitflags::bitflags;
use tauri::{
  generate_handler, AppHandle, LogicalPosition, Manager, Monitor, State, Window, WindowBuilder,
  WindowUrl,
};

pub static kCGDesktopWindowLevel: i32 = -2147483623;

pub fn run() {
  let app = tauri::Builder::default()
    .manage(WindowPositions(Mutex::default()))
    .invoke_handler(generate_handler![update_location])
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

  #[allow(clippy::single_match)]
  app.run(|app_handle, event| match event {
    tauri::RunEvent::Ready => create_wallpaper_windows(app_handle),
    _ => {}
  })
}

#[tauri::command]
fn update_location(window: Window, poses: State<WindowPositions>) {
  let poses = poses.0.lock().unwrap();
  let monitor = poses.get(window.label());

  if let Some(monitor) = monitor {
    set_window_loc_to_monitor(&window, monitor);
  }
}

fn create_wallpaper_windows(app_handle: &AppHandle) {
  let main_window = create_wallpaper_window(app_handle, None);
  let main_monitor = main_window.current_monitor().unwrap().unwrap();

  for monitor in main_window.available_monitors().unwrap() {
    if monitor.position() == main_monitor.position() {
      continue;
    }

    create_wallpaper_window(app_handle, Some(monitor));
  }

  for (label, _) in app_handle.windows() {
    println!("{}", label);
  }
}

fn create_wallpaper_window(app_handle: &AppHandle, monitor: Option<Monitor>) -> Window {
  let (window_label, pos) = if let Some(monitor) = &monitor {
    let pos = monitor.position();
    let size = monitor.size();
    (
      format!(
        "wallpaper__{}_{}__{}_{}",
        pos.x, pos.y, size.width, size.height
      ),
      pos.to_logical(monitor.scale_factor()),
    )
  } else {
    ("wallpaper__main".to_string(), LogicalPosition::new(0, 0))
  };

  let window = WindowBuilder::new(
    app_handle,
    window_label.clone(),
    WindowUrl::App("index.html".parse().unwrap()),
  )
  .decorations(false)
  .position(pos.x as f64, pos.y as f64)
  .build()
  .unwrap_or_else(|_| panic!("error while building wallpaper window ({window_label})"));

  // save monitor info
  let monitor = if let Some(monitor) = monitor {
    monitor
  } else {
    window.current_monitor().unwrap().unwrap()
  };
  let poses = app_handle.state::<WindowPositions>();
  poses.0.lock().unwrap().insert(window_label, monitor);

  // set to wallpaper level
  #[cfg(target_os = "macos")]
  unsafe {
    use cocoa::{
      appkit::{NSWindow, NSWindowCollectionBehavior},
      base::id,
      foundation::NSInteger,
    };

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
      pub(crate) fn CGWindowLevelForKey(key: i32) -> NSInteger;
    }

    let ns_window = window.ns_window().unwrap() as id;

    ns_window.setCollectionBehavior_(
      NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary,
    );

    ns_window.setLevel_(CGWindowLevelForKey(CGWindowLevelKey::kCGDesktopWindowLevelKey.bits) - 1)
  }

  window
}

fn set_window_loc_to_monitor(window: &Window, monitor: &Monitor) {
  window
    .set_position(*monitor.position())
    .unwrap_or_else(|_| {
      panic!(
        "error while setting position of wallpaper window ({})",
        window.label()
      )
    });
  window.set_size(*monitor.size()).unwrap_or_else(|_| {
    panic!(
      "error while setting size of wallpaper window ({})",
      window.label()
    )
  });
}

bitflags! {
  struct CGWindowLevelKey: i32 {
    const kCGBaseWindowLevelKey = 0;
    const kCGMinimumWindowLevelKey = 1;
    const kCGDesktopWindowLevelKey = 2;
    const kCGBackstopMenuLevelKey = 3;
    const kCGNormalWindowLevelKey = 4;
    const kCGFloatingWindowLevelKey = 5;
    const kCGTornOffMenuWindowLevelKey = 6;
    const kCGDockWindowLevelKey = 7;
    const kCGMainMenuWindowLevelKey = 8;
    const kCGStatusWindowLevelKey = 9;
    const kCGModalPanelWindowLevelKey = 10;
    const kCGPopUpMenuWindowLevelKey = 11;
    const kCGDraggingWindowLevelKey = 12;
    const kCGScreenSaverWindowLevelKey = 13;
    const kCGMaximumWindowLevelKey = 14;
    const kCGOverlayWindowLevelKey = 15;
    const kCGHelpWindowLevelKey = 16;
    const kCGUtilityWindowLevelKey = 17;
    const kCGDesktopIconWindowLevelKey = 18;
    const kCGNumberOfWindowLevelKeys = 19;
  }
}

pub struct WindowPositions(Mutex<HashMap<String, Monitor>>);
