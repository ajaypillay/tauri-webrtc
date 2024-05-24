// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


use tauri::Manager;
#[cfg(target_os = "linux")]
use webkit2gtk::{SettingsExt, WebViewExt};

#[cfg(target_os = "linux")]
fn enable_web_features(settings: &webkit2gtk::Settings) {
    println!("enabling webrtc");
    settings.set_enable_webrtc(true);
    settings.set_enable_media_stream(true);
    // settings.set_enable_mediasource(true);
    // settings.set_enable_media(true);
    // settings.set_enable_media_capabilities(true);
    // settings.set_enable_encrypted_media(true);
    // settings.set_enable_mock_capture_devices(true);
    // settings.set_media_playback_requires_user_gesture(false);
    // settings.set_media_playback_allows_inline(true);
    // settings.set_media_content_types_requiring_hardware_support(None);
    // settings.set_disable_web_security(true);
}

#[cfg(target_os = "linux")]
fn allow_all_permissions(webview: &webkit2gtk::WebView) {
    use webkit2gtk::PermissionRequestExt;
    // Allow all permission requests for debugging
    let _ = webview.connect_permission_request(move |_, request| {
        request.allow();
        true
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.webview_windows().values().for_each(|webview_window| {
                #[cfg(target_os = "linux")]
                if let Err(e) = webview_window.with_webview(|webview| {
                    if let Some(settings) = webview.inner().settings() {
                        enable_web_features(&settings);
            
                        allow_all_permissions(&webview.inner());
                    }
                }) {
                    eprintln!("Error configuring webview: {:?}", e);
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
