// <meta property="og:title" content="The Rock" />
// <meta property="og:type" content="video.movie" />
// <meta property="og:url" content="https://www.imdb.com/title/tt0117500/" />
// <meta property="og:image" content="https://ia.media-imdb.com/images/rock.jpg" />

use coyote::{tmpl, Component};

pub struct OpenGraphParams {
    pub title: Component,
    pub r#type: Component,
    pub url: Component,
    pub image: Component,
    pub body: Component,
}

