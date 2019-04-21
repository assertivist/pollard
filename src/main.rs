#![windows_subsystem = "windows"]

extern crate web_view;

use web_view::*;


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
        .size(632, 600)
        .resizable(false)
        .debug(true)
        .user_data(0)
        .invoke_handler(|_webview, _arg| {
        	match _arg {
        		"init" => {

        		},
        		"test" => {
        			eprint!("test invoked");
        		},
        		_ => unimplemented!(),
        	};
        	Ok(())
        })
        .build()
        .unwrap();

    webview.run().unwrap();
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}