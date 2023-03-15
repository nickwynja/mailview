use web_view::*;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use open;
use soup::prelude::*;
use image_base64;

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let mut file = File::open(arg1).expect("Unable to open the file");
        let mut buf = vec![];
        file.read_to_end (&mut buf).unwrap();
        let mut contents = String::from_utf8_lossy(&buf).to_string();

        let soup = Soup::new(&contents);

        for (_i, link) in soup.tag("img").find_all().enumerate() {
            let src = link.get("src").expect("Couldn't find link with 'href' attribute");
            if ! src.starts_with("http") {
                // @TODO: catch err so this works offline
                let img = &image_base64::to_base64(&src);
                contents = contents.replace(&src, img);
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

        web_view::builder()
            .title("")
            .content(Content::Html(html))
            .size(800, 900)
            .resizable(true)
            .debug(true)
            .user_data(())
            .invoke_handler(|webview, arg| {
                if arg == "exit" {
                    webview.exit();
                } else if arg.starts_with("bg") {
                    let split = arg.split(" ");
                    let vec: Vec<&str> = split.collect();
                    let url = vec[1];
                    println!("{:?}", url);
                    open::that(url).unwrap()
                } else {
                    webview.exit();
                    open::that(arg).unwrap()
                }
                Ok(())
            })
            .run()
            .unwrap();
    }
}
