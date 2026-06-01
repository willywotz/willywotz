use axum::{routing::get, Router, response::{Html}};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new()
        .route("/calendar", get(calendar))
        .fallback(|| async { "Hello World!" })
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}

pub async fn calendar() -> Html<&'static str> {
    Html(r#"<!doctypehtml><html lang=en><meta charset=utf-8><meta content="width=device-width,initial-scale=1"name=viewport><title>Calendar</title><style>body,html{width:100%;height:100%;margin:0;padding:0}iframe{display:block;width:100%;height:100%;border:0}</style><iframe src="https://calendar.google.com/calendar/embed?wkst=1&ctz=Asia%2FBangkok&showPrint=0&src=ZTkxN2EzYjEyMWEzZjg3NjA3OWI2ZDE4YmVmYzM0NmE5ZGJiNTljZjRhOWY1ZDYwOTI4NGFiZjZhNjc3MzVlMUBncm91cC5jYWxlbmRhci5nb29nbGUuY29t&color=%233f51b5"title="Google Calendar"></iframe>"#)
}
