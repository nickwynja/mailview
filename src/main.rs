use web_view::*;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use open;

// @TODO: https://github.com/webview/webview/issues/44#issuecomment-350342541
//        Properly load local cid images

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let mut file = File::open(arg1).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");

        let html = format!(
            r##"
            <!doctype html>
            <html>
                <head>
                    <meta charset="utf-8">
                    <base target="_top">
                    {scripts}
                </head>
                <body>
                    {body}
                    <!--[if lt IE 9]>
                    <div class="ie-upgrade-container">
                        <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
                        <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
                    </div>
                    <![endif]-->
                    <!--[if gte IE 9 | !IE ]> <!-->
                    <![endif]-->
                </body>
            </html>
            "##,
            scripts = inline_script(include_str!("./app.js")),
            body = contents
        );

        web_view::builder()
            .title("Email")
            .content(Content::Html(html))
            .size(740, 768)
            .resizable(true)
            .debug(true)
            .user_data(())
            .invoke_handler(|webview, arg| {
                match arg {
                    "exit" => webview.exit(),
                    _ => open::that(arg).unwrap()
                }
                Ok(())
            })
            .run()
            .unwrap();
    }
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
