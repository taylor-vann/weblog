use coyote::{Component, tmpl};

use crate::document::document_frame;

// metas can be computed
// no scripts yet?

pub fn home() -> Component {
    tmpl("hai :3", [])
}
