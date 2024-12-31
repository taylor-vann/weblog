use coyote::{attr_val, tmpl, Component};

use components::document::{document_frame, lang_en, metas};

const title: &str = "taylorvann - home";
const description: &str = "Software engineer, hobbyist, blogger.";

pub fn page() -> Component {
    document_frame(
        lang_en(),
        metas(title, description),
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
            <p>hai :3</p>
        </main>
        <footer></footer>
        ",
        [],
    )
}
