use coyote::Component::None;
use coyote::{attr_val, tmpl, Component};

use crate::document::document_frame;

// metas can be computed
// no scripts yet?

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

pub fn page() -> Component {
    document_frame(
        attr_val("lang", "en-us"),
        None,   // meta
        None,   // styles
        None,   // scripts
        body(), // body
    )
}
