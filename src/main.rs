use std::env;
use wry;
use open;
use std::fs::File;
use std::io::Read;
use soup::prelude::*;
use image_base64;
use std::path::Path;

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn main() -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            dpi::LogicalSize,
            window::{Window, WindowBuilder},
        },
        webview::WebViewBuilder,
    };

  enum UserEvents {
    CloseWindow,
  }

  let arg = env::args().nth(1).unwrap();
  let event_loop = EventLoop::<UserEvents>::with_user_event();
  let window = WindowBuilder::new()
    .with_title("")
    .build(&event_loop)
    .unwrap();

  let mut file = File::open(arg).expect("Unable to open the file");
  let mut buf = vec![];
  file.read_to_end(&mut buf).unwrap();
  let mut contents = String::from_utf8_lossy(&buf).to_string();

  let soup = Soup::new(&contents);

  for (_i, link) in soup.tag("img").find_all().enumerate() {
      let src = link.get("src").unwrap_or_default();
      if src != "" && ! src.starts_with("http") && ! src.starts_with("data:image") && ! src.starts_with("cid:"){
          if Path::new(&src).exists() {
              let img = &image_base64::to_base64(&src);
              contents = contents.replace(&src, &img.clone().unwrap());
          }
      }
  }

  let html = format!(
      r##"
    <!doctype html>
    <html>
        <head>
            <meta charset="utf-8">
            {styles}
            {scripts}
        </head>
        <body>{body}</body>
    </html>
    "##,
    styles = inline_style(include_str!("./app.css")),
    scripts = inline_script(include_str!("./app.js")),
    body = contents,
);

  window.set_inner_size(LogicalSize::new(700, 900));

  let proxy = event_loop.create_proxy();

  let handler = move |_window: &Window, req: String| {
    if req == "close" {
      let _ = proxy.send_event(UserEvents::CloseWindow);
    } else if req.starts_with("bg") {
        let split = req.split(" ");
        let vec: Vec<&str> = split.collect();
        let url = vec[1];
        open::that(url).unwrap()
    } else {
        let _ = proxy.send_event(UserEvents::CloseWindow);
        open::that(&req).unwrap()
    }
  };

  let _webview =
      WebViewBuilder::new(window)
      .unwrap()
      .with_html(html)?
      .with_ipc_handler(handler)
      .with_accept_first_mouse(true)
      .build()?;


  _webview.evaluate_script(include_str!("./app.js"))?;

  let mut webview = Some(
      _webview,
      );

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      }
      | Event::UserEvent(UserEvents::CloseWindow) => {
        let _ = webview.take();
        *control_flow = ControlFlow::Exit
      }
      _ => (),
    }
  });
}


