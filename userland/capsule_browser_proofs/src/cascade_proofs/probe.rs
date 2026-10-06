// NONOS Operating System (AGPL-3.0-or-later)
//! The cascade alone, for proofs about computed values: parse a page, run
//! its <style> text through the cascade, and look styles up by element id.

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::{collect_css, compute, Computed};
use crate::browser::dom::{self, Dom};

pub struct Styles {
    pub dom: Dom,
    pub styles: Vec<Computed>,
    pub bg_images: Vec<Option<String>>,
}

impl Styles {
    pub fn of(html: &str) -> Styles {
        let dom = dom::parse(html.as_bytes());
        let css = collect_css(&dom);
        let out = compute(&dom, &css);
        let styles = (0..dom.nodes.len()).map(|i| out.styles[i]).collect();
        Styles { styles, bg_images: out.bg_images, dom }
    }

    /* The DOM index of the element with this id. */
    pub fn node(&self, id: &str) -> usize {
        let hit = self.dom.nodes.iter().position(|n| n.attr("id") == Some(id));
        hit.unwrap_or_else(|| panic!("no element #{id}"))
    }

    pub fn get(&self, id: &str) -> &Computed {
        &self.styles[self.node(id)]
    }

    pub fn bg_image(&self, id: &str) -> Option<&str> {
        self.bg_images[self.node(id)].as_deref()
    }
}
