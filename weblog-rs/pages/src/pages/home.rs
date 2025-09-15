use coyote::{attr_val, tmpl, Component};

use crate::components::document::{document, DocumentParams};

const TITLE: &str = "taylorvann - home";
const DESCRIPTION: &str = "Software engineer, hobbyist, blogger.";

pub fn page() -> Component {
    document(DocumentParams {
        language: lang_en(),
        metas: metas(TITLE, DESCRIPTION),
        styles: Component::None,  // styles
        scripts: Component::None, // scripts
        body: body(),
    })
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

fn body() -> Component {
    tmpl(
        "
        <header></header>
        <main>
            <p>Arrroooo!!</p>
        </main>
        <footer></footer>
        ",
        [],
    )
}
