#![windows_subsystem = "windows"]

extern crate web_view;
extern crate serde_derive;
extern crate serde_json;

use web_view::*;
use serde_derive::Deserialize;

fn main() {
    let html = format!(r#"
    <!DOCTYPE html>
    <html>
        <head>
        <title>Pollard</title>
        {styles}
        </head>
        <body>
            <div id="elm"></div>
        </body>
        {scripts}
    </html>
    "#,
    styles = inline_style(include_str!("../style.css")),
    scripts = inline_script(include_str!("../elm.js")) 
              + &inline_script(include_str!("../ports.js"))
    );

    let webview = web_view::builder()
        .title("Pollard")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(0)
        .invoke_handler(|_webview, _arg| {
        	use self::Cmd::*;
            match serde_json::from_str(_arg).unwrap() {
                Init => (),
                Log { text } => println!("{}", text),
                Login { username, password } => (),
                SendChat { text } => println!("chat: {}", text),
                Logout => ()
            }
            Ok(())
        })
        .build()
        .unwrap();

    webview.run().unwrap();
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Log { text : String },
    Login { username: String, password: String},
    SendChat { text: String },
    Logout
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}