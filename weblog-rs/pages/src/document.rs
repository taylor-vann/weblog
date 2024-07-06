use coyote::{Component, tmpl};

pub fn document_frame(
    metas: Component,
    styles: Component,
    javascript: Component,
    content: Component,
) -> Component {
    tmpl("
        <!DOCTYPE html>
        <html lang=\"en-US\">
            <head>
                ${}
                ${}
                ${}
            </head>
            <body>
                ${}
            </body>
        </html>",
        [
            styles,
            javascript,
            content,
        ]
    )
}

pub fn template(
    attrs: Component,
    shadow_dom: Component,
    light_dom: Component, 
) -> Component {
    tmpl("
        <template {}>
            {}
            {}
        </template>",
        [
            attrs,
            shadow_dom,
            light_dom,
        ]
    )
}
