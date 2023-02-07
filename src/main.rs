use web_view::*;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::PathBuf;




fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let path = PathBuf::from(&arg1);
        let dir = path.parent().unwrap();
        assert!(env::set_current_dir(&dir).is_ok());
        println!("Successfully changed working directory to {}!", dir.display());
        let mut file = File::open(arg1).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");

        let html = format!(
            r##"
            <!doctype html>
            <html>
                <head>
                    <meta charset="utf-8">
                    <script>
                    document.onkeydown = function(event) {{
                        switch (event.key) {{
                           case 'q':
                                external.invoke('exit')
                              break;
                           case 'G':
                                window.scrollTo(0, 0)
                              break;
                           case 'g':
                                window.scrollTo(0, document.body.scrollHeight)
                              break;
                           case 'k':
                               window.scrollBy(0, -50);
                              break;
                           case 'j':
                               window.scrollBy(0, 50);
                              break;
                        }}
                    }};
                    document.addEventListener("DOMContentLoaded", function() {{
                    for (var i= document.images.length; i-->0;)
                    if (document.images[i].src.startsWith('cid')) {{
    document.images[i].src = "file://" + document.images[i].src;
    }}

                    }})
                    </script>
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
            body = contents,
        );


        println!("{}", html);


        web_view::builder()
            .title("Email")
            .content(Content::Html(html))
            .size(740, 768)
            .resizable(true)
            .debug(false)
            .user_data(())
            .invoke_handler(|webview, arg| {
                match arg {
                    "exit" => webview.exit(),
                    _ => (),
                }
                Ok(())
            })
            .run()
            .unwrap();
    }
}
// @TODO: https://github.com/webview/webview/issues/44#issuecomment-350342541
//        Properly load local cid images
