use super::{ApiRequest, ApiResponse};
use directories::UserDirs;
use serde_json::{json, Value};

pub fn call(request: ApiRequest) -> ApiResponse {
    return match request.method.as_str() {
        "info" => platform(request),
        "dirs" => dirs(request),
        "sep" => sep(request),
        "eol" => eol(request),
        _ => ApiResponse::err(request.callback_id,"Method not found"),
    };
}

fn platform(request: ApiRequest) -> ApiResponse {
    let info = os_info::get();

    return ApiResponse::ok(request.callback_id,json!({
        "os": info.os_type().to_string(),
        "arch": std::env::consts::ARCH.to_string(),
        "version": info.version().to_string(),
    }));
}

fn unwrap_path(path: &std::path::Path) -> Value {
    return Value::String(path.to_str().unwrap_or("").to_string());
}

fn unwrap_path_opt(path_result: Option<&std::path::Path>) -> Value {
    if path_result.is_none() {
        return Value::Null;
    }
    return Value::String(path_result.unwrap().to_str().unwrap_or("").to_string());
}

fn dirs(request: ApiRequest) -> ApiResponse {
    let user_dirs = UserDirs::new().unwrap();

    return ApiResponse::ok(request.callback_id,json!({
        "home": unwrap_path(user_dirs.home_dir()),
        "audio": unwrap_path_opt(user_dirs.audio_dir()),
        "desktop": unwrap_path_opt(user_dirs.desktop_dir()),
        "document": unwrap_path_opt(user_dirs.document_dir()),
        "download": unwrap_path_opt(user_dirs.download_dir()),
        "font": unwrap_path_opt(user_dirs.font_dir()),
        "picture": unwrap_path_opt(user_dirs.picture_dir()),
        "public": unwrap_path_opt(user_dirs.public_dir()),
        "template": unwrap_path_opt(user_dirs.template_dir()),
        "video": unwrap_path_opt(user_dirs.video_dir()),
    }));
}

fn sep(request: ApiRequest) -> ApiResponse {
    ApiResponse::ok(request.callback_id, json!({
        "sep":  std::path::MAIN_SEPARATOR
    }))
}

fn eol(request: ApiRequest) -> ApiResponse {
    #[cfg(target_os = "windows")]
    let eol = "\r\n";
    #[cfg(not(target_os = "windows"))]
    let eol = "\n";

    ApiResponse::ok(request.callback_id, json!({
        "eol": eol,
    }))
}
