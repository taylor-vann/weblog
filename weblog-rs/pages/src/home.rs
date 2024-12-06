use coyote::{attr_val, tmpl, Component};

use components::document::{document_frame, lang_en, metas};

const title: &str = "Taylor Vann dot com";
const description: &str = "I make food, art, and code in no particular order.";

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
            <p>I'm Taylor Vann</p>
            <p>Yet another software engineer.</p>
        </main>
        <footer></footer>
        ",
        [],
    )
}
