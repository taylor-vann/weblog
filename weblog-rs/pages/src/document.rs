use coyote::{tmpl, Component};

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
