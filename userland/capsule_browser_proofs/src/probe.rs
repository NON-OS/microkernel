// NONOS Operating System (AGPL-3.0-or-later)
//! Find laid-out boxes by the id of the element behind them, for proofs
//! about one element's geometry.

use crate::browser::dom::Dom;
use crate::browser::layout::boxmodel::{BoxDocument, Content, Fragment};
use crate::render::render_full;

/* A page laid out at a viewport, with no image sizes known. */
pub struct Page {
    pub dom: Dom,
    pub doc: BoxDocument,
}

impl Page {
    pub fn at(html: &str, vp: (u32, u32)) -> Page {
        let (dom, doc) = render_full(html, vp, &|_| None);
        Page { dom, doc }
    }

    /* The DOM index of the element with this id. */
    pub fn node(&self, id: &str) -> usize {
        let hit = self.dom.nodes.iter().position(|n| n.attr("id") == Some(id));
        hit.unwrap_or_else(|| panic!("no element #{id}"))
    }

    /* The first fragment painted for the element with this id: its own box. */
    pub fn frag(&self, id: &str) -> &Fragment {
        let n = self.node(id);
        let hit = self.doc.frags.iter().find(|f| f.node == n);
        hit.unwrap_or_else(|| panic!("#{id} painted nothing"))
    }

    /* [x, y, w, h] of that fragment. */
    pub fn rect(&self, id: &str) -> [i32; 4] {
        let f = self.frag(id);
        [f.x, f.y, f.w, f.h]
    }

    /* The first text fragment that paints `word`. */
    pub fn word(&self, word: &str) -> &Fragment {
        let hit = self
            .doc
            .frags
            .iter()
            .find(|f| matches!(&f.content, Content::Text { text, .. } if text == word));
        hit.unwrap_or_else(|| panic!("{word} not laid out"))
    }
}
