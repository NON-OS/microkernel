// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::html::tokenizer::{Tag, TextMode};

use super::super::super::node::Ns;
use super::super::super::quirks::Quirks;
use super::super::ops::link::Loc;
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// A builder for the HTML fragment parsing algorithm (13.4): content
    /// parsed as if it were inside an element named `tag` in `ns`, which is
    /// what innerHTML means. The context element is kept outside the tree.
    pub fn fragment(tag: &str, ns: Ns, quirks: Quirks) -> Self {
        let mut b = Builder::document();
        b.dom.quirks = quirks;
        if let Some(root) = b.create(Tag::implied("html"), Ns::Html) {
            b.link(Loc::end(0), root);
            b.push_open(root);
        }
        let context =
            Tag { name: Cow::Owned(String::from(tag)), attrs: Vec::new(), self_closing: false };
        b.context = b.create(context, ns);
        if b.context_is("template") {
            b.tmpl.push(Mode::InTemplate);
        }
        b.reset_mode();
        b
    }
}

/// The tokenizer state a fragment starts in, from its context element.
pub fn fragment_state(tag: &str, ns: Ns) -> TextMode {
    if ns != Ns::Html {
        return TextMode::Data;
    }
    match tag {
        "title" | "textarea" => TextMode::Rcdata,
        "style" | "xmp" | "iframe" | "noembed" | "noframes" => TextMode::Rawtext,
        "script" => TextMode::ScriptData,
        "plaintext" => TextMode::Plaintext,
        _ => TextMode::Data,
    }
}
