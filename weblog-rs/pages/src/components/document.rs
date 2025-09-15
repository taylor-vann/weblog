use coyote::{tmpl, Component};

pub struct DocumentParams {
    pub language: Component,
    pub metas: Component,
    pub styles: Component,
    pub scripts: Component,
    pub body: Component,
}

pub fn document(params: DocumentParams) -> Component {
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
        [
            params.language,
            params.metas,
            params.styles,
            params.scripts,
            params.body,
        ],
    )
}
