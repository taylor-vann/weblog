use coyote::{attr_val, tmpl, Component};

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

pub fn favicon() -> Component {
    tmpl(
        "
        <link rel=icon href=/favicon.svg>
        <link rel=mask-icon href=/mask-icon.svg color=#000000>
        <link rel=apple-touch-icon href=/apple-touch-icon.png>
        "
    )
}

pub fn build_links() -> Component {
    tmpl(
        "
        <link rel=manifest href=/manifest.webmanifest>
        "
    )
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
