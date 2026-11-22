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

//! Plain-text and error documents wrap at the pane width they are handed, in CSS px,
//! not at the manifest's fixed width. The margin is read off the layout (`wrap::MARGIN`
//! is private to it). Its own binary.

use capsule_browser_proofs::browser::fetch::render_error::render_error;
use capsule_browser_proofs::browser::fetch::render_response::{render_response, Rendered};
use capsule_browser_proofs::browser::http::response::{ContentKind, Response};
use capsule_browser_proofs::browser::layout::doc::RenderDocument;

const WIDTHS: [u32; 3] = [400, 683, 1360];
const PROSE: &str = "The certificate presented by the server was issued for a different \
    host name than the one requested, so the connection was closed before any content \
    from it was read or shown in this pane.";

fn inside(doc: &RenderDocument, w: u32) {
    assert!(!doc.lines.is_empty(), "nothing laid out at {w}");
    let margin = doc.lines[0].spans[0].x;
    assert!(margin > 0, "the layout's own inset is its first span's x");
    for s in doc.lines.iter().flat_map(|l| &l.spans) {
        assert!(s.x + s.w <= w - margin, "{:?} at x={} w={} overflows {w}", s.text, s.x, s.w);
    }
}

fn text(kind: ContentKind, w: u32) -> RenderDocument {
    let resp = Response {
        status: 200,
        body: PROSE.repeat(3).into_bytes(),
        location: None,
        content_kind: kind,
    };
    match render_response(&resp, w).0 {
        Rendered::Lines(doc) => doc,
        _ => panic!("a text response renders as lines"),
    }
}

#[test]
fn error_page_wraps_at_the_pane_width() {
    for w in WIDTHS {
        inside(&render_error(PROSE, w), w);
    }
    assert!(render_error(PROSE, 400).lines.len() > render_error(PROSE, 1360).lines.len());
}

#[test]
fn plain_text_wraps_at_the_pane_width() {
    for w in WIDTHS {
        inside(&text(ContentKind::Text, w), w);
    }
    assert!(text(ContentKind::Text, 400).lines.len() > text(ContentKind::Text, 1360).lines.len());
}

#[test]
fn unsupported_notice_wraps_at_the_pane_width() {
    for w in WIDTHS {
        inside(&text(ContentKind::Unsupported, w), w);
    }
}
