use coyote::{tmpl, Component};

use components::document::{document_frame, lang_en, metas};

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
