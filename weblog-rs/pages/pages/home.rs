use coyote::{tmpl, Component};

use components::document::{document, metas};

const TITLE: &str = "taylorvann - home";
const DESCRIPTION: &str = "Software engineer, hobbyist, blogger.";

pub fn page() -> Component {
    document_frame(
        lang_en(),
        metas(TITLE, DESCRIPTION),
        Component::None, // styles
        Component::None, // scripts
        body(),
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