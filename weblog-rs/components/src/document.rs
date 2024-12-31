use coyote::{attr_val, tmpl, unescaped_text, Component};

pub fn document_frame(
    language: Component,
    metas: Component,
    styles: Component,
    scripts: Component,
    content: Component,
) -> Component {
    tmpl(
        "
        <!DOCTYPE html>
        <html {}>
            <head>
                {}
                {}
                {}
            </head>
            <body>
                {}
            </body>
        </html>",
        [language, metas, styles, scripts, content],
    )
}

pub fn lang_en() -> Component {
    attr_val("lang", "en-us")
}

pub fn metas(title: &str, description: &str) -> Component {
    tmpl(
        "
        <meta charset=utf-8>
        <meta name=viewport content=\"width=device-width, initial-scale=1\">
        <meta {}>
        <meta {}>
        ",
        [
            attr_val("title", title),
            attr_val("description", description),
        ],
    )
}
