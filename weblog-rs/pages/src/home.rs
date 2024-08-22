use coyote::Component::None;
use coyote::{attr_val, tmpl, Component};

use components::document::{document_frame, lang_en, metas};

pub fn page() -> Component {
    document_frame(
        lang_en(),
        metas(),
        None,   // styles
        None,   // scripts
        content(),
    )
}


fn content() -> Component {
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
