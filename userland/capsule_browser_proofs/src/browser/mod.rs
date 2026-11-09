// NONOS Operating System (AGPL-3.0-or-later)
//! The engine's own module tree, compiled from the capsule's source. These
//! used to be hand-written mirrors listing every file, which drifted: the dom
//! mirror had been missing `measure` and `serialize` for a while. Pointing at
//! the real mod.rs keeps one list of what the engine is made of.

#[path = "../../../capsule_browser/src/browser/css/mod.rs"]
pub mod css;
#[path = "../../../capsule_browser/src/browser/dom/mod.rs"]
pub mod dom;
#[path = "../../../capsule_browser/src/browser/event/dom_print.rs"]
pub mod dom_print;
#[path = "../../../capsule_browser/src/browser/fonts/mod.rs"]
pub mod fonts;
#[path = "../../../capsule_browser/src/browser/html/mod.rs"]
pub mod html;
#[path = "../../../capsule_browser/src/browser/layout/mod.rs"]
pub mod layout;
#[path = "../../../capsule_browser/src/browser/omnibox/mod.rs"]
pub mod omnibox;
#[path = "../../../capsule_browser/src/browser/reader/mod.rs"]
pub mod reader;
#[path = "../../../capsule_browser/src/browser/shell/mod.rs"]
pub mod shell;
#[path = "../../../capsule_browser/src/browser/ui/mod.rs"]
pub mod ui;
#[path = "../../../capsule_browser/src/browser/url/mod.rs"]
pub mod url;
#[path = "../../../capsule_browser/src/browser/short_name.rs"]
pub mod short_name;
#[path = "../../../capsule_browser/src/browser/paint/tokens/mod.rs"]
pub mod tokens;
/* The toolbar's controls and its route badge, drawn from shared widgets
 * with no window state, so the proofs press what they paint. */
#[path = "../../../capsule_browser/src/browser/paint/toolbar"]
pub mod toolbar {
    pub mod badge;
    pub mod buttons;
    pub mod look;
}
/* The tab strip and the sidebar rows, likewise. */
#[path = "../../../capsule_browser/src/browser/paint/tabbar"]
pub mod tabbar {
    pub mod marks;
    pub mod strip;
}
#[path = "../../../capsule_browser/src/browser/paint/sidebar"]
pub mod sidebar {
    pub mod brand;
    pub mod look;
    pub mod nav;
    pub mod paint;
}
/* The settings page's buttons, likewise, from what they show. */
#[path = "../../../capsule_browser/src/browser/paint/settings"]
pub mod settings {
    pub mod caption;
    pub mod controls;
    pub mod field;
    pub mod look;
}
/* The reader settings card, likewise, from the reader's settings. */
#[path = "../../../capsule_browser/src/browser/paint/reader"]
pub mod reader_card {
    pub mod controls;
    pub mod panel;
}
/* The sidebar's painter names its siblings under `paint`, as the capsule
 * lays them out. */
pub mod paint {
    pub use super::{sidebar, tokens};
}

/* js keeps its mirror: the interpreter modules under test are private to the
 * capsule and nothing outside the engine has cause to reach them. */
pub mod event;
pub mod image;
pub mod js;
pub mod manifest;

#[path = "../../../capsule_browser/src/browser/net/recv_pending.rs"]
pub mod recv_pending;

/* The fetch machine: its socket calls behind a trait, its TLS the real one. */
pub mod fetch;
#[path = "../../../capsule_browser/src/browser/cookie/mod.rs"]
pub mod cookie;
#[path = "../../../capsule_browser/src/browser/http/mod.rs"]
pub mod http;
pub mod net;
#[cfg(test)]
#[path = "../../../capsule_browser/src/browser/proxy/said.rs"]
mod said;
#[cfg(test)]
mod said_tests;
pub use tls_proofs as tls13;
#[path = "../../../capsule_browser/src/browser/fetch/enqueue_css/sheet.rs"]
pub mod sheet;
