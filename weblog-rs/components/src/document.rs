use coyote::{attr_val, tmpl, unescaped_text, Component};

pub fn document_frame(
    language: Component,
    metas: Component,
    styles: Component,
    javascript: Component,
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
        [language, metas, styles, javascript, content],
    )
}

pub fn template(attrs: Component, shadow_dom: Component, light_dom: Component) -> Component {
    tmpl(
        "
        <template {}>
            {}
            {}
        </template>",
        [attrs, shadow_dom, light_dom],
    )
}

pub fn lang_en() -> Component {
    attr_val("lang", "en-us")
}

pub fn metas() -> Component {
    tmpl(
        "
        <meta charset=utf-8>
        <meta name=viewport content=\"width=device-width, initial-scale=1\">
        ",
        [],
    )
}

pub fn styles() -> Component {
    unescaped_text("")
}
