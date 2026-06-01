use worker::*;

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    match req.path().as_str() {
        "/calendar" => Response::from_html(
            r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Calendar</title>
    <style>
      html, body {
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 0;
      }
      iframe {
        display: block;
        width: 100%;
        height: 100%;
        border: 0;
      }
    </style>
  </head>
  <body>
    <iframe title="Google Calendar" src="https://calendar.google.com/calendar/embed?wkst=1&ctz=Asia%2FBangkok&showPrint=0&src=ZTkxN2EzYjEyMWEzZjg3NjA3OWI2ZDE4YmVmYzM0NmE5ZGJiNTljZjRhOWY1ZDYwOTI4NGFiZjZhNjc3MzVlMUBncm91cC5jYWxlbmRhci5nb29nbGUuY29t&color=%233f51b5"></iframe>
  </body>
</html>"#,
        ),
        _ => Response::ok("Hello World!"),
    }
}
