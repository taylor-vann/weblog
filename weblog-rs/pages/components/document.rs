use coyote::{attr_val, tmpl, Component};

pub fn document(
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

