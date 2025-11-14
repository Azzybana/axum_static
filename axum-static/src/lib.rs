use axum::{
    Router,
    body::Body,
    http::Request,
    middleware::{Next, from_fn},
    response::Response,
    routing::get_service,
};
use std::path::Path;
use tower_http::services::ServeDir;

pub async fn content_type_middleware(request: Request<Body>, next: Next) -> Response {
    let uri = request.uri().to_owned();
    let path = uri.path();

    // Extract the extension before awaiting to avoid holding a borrow across await points.
    let extension = path.rsplit('.').next().map(str::to_ascii_lowercase);

    let mut response = next.run(request).await;

    let content_type = if let Some(ext) = extension {
        match ext.as_str() {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "text/javascript",
            "json" => "application/json",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "ico" => "image/x-icon",
            "ttf" => "font/ttf",
            "woff" => "font/woff",
            "woff2" => "font/woff2",
            "eot" => "application/vnd.ms-fontobject",
            "otf" => "font/otf",
            "txt" => "text/plain",
            "pdf" => "application/pdf",
            "doc" => "application/msword",
            "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "xls" => "application/vnd.ms-excel",
            "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "ppt" => "application/vnd.ms-powerpoint",
            "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            "xml" => "application/xml",
            "zip" => "application/zip",
            "rar" => "application/x-rar-compressed",
            "7z" => "application/x-7z-compressed",
            "gz" => "application/gzip",
            "tar" => "application/x-tar",
            "swf" => "application/x-shockwave-flash",
            "flv" => "video/x-flv",
            "avi" => "video/x-msvideo",
            "mov" => "video/quicktime",
            "mp4" | "f4v" | "f4p" | "f4a" | "f4b" => "video/mp4",
            "mp3" => "audio/mpeg",
            "wav" => "audio/x-wav",
            "ogg" => "audio/ogg",
            "webm" => "video/webm",
            "mpg" | "mpeg" | "mpe" | "mp2" => "video/mpeg",
            "m4v" => "video/x-m4v",
            "3gp" => "video/3gpp",
            "3g2" => "video/3gpp2",
            "mkv" | "amv" => "video/x-matroska",
            "m3u" => "audio/x-mpegurl",
            "m3u8" => "application/vnd.apple.mpegurl",
            "ts" => "video/mp2t",
            "webp" => "image/webp",
            "bmp" => "image/bmp",
            "tif" | "tiff" => "image/tiff",
            "psd" => "image/vnd.adobe.photoshop",
            "ai" | "eps" | "ps" => "application/postscript",
            "dwg" => "image/vnd.dwg",
            "dxf" => "image/vnd.dxf",
            "rtf" => "application/rtf",
            "odt" => "application/vnd.oasis.opendocument.text",
            "ods" => "application/vnd.oasis.opendocument.spreadsheet",
            "wasm" => "application/wasm",
            _ => "application/octet-stream",
        }
    } else {
        "unknown"
    };

    if let Ok(content_type) = content_type.parse() {
        response.headers_mut().insert("Content-Type", content_type);
    }

    response
}

pub fn static_router<P: AsRef<Path>>(path: P) -> Router {
    #[cfg(feature = "handle_error")]
    async fn handle_error(err: std::io::Error) -> impl IntoResponse {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("static router IO error: {:?}", err),
        )
            .into_response()
    }

    let serve_dir = ServeDir::new(path).append_index_html_on_directories(true);
    #[cfg(feature = "handle_error")]
    let serve_dir = get_service(serve_dir).handle_error(handle_error);
    #[cfg(not(feature = "handle_error"))]
    let serve_dir = get_service(serve_dir);

    Router::new()
        .fallback_service(serve_dir)
        .layer(from_fn(content_type_middleware))
}
