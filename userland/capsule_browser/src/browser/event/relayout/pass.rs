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

use crate::browser::css::{collect_css, CssCache};
use crate::browser::layout::boxmodel::{build, layout};
use crate::browser::state::State;

/* One relayout. QuickJS is on when the page has an engine; whether its
 * scripts threw is not known, as nonos_qjs returns an exception's message
 * and a result alike as text, so the <noscript> policy gets None and
 * judges a failed page by what it left: no body text outside it. */
pub(super) fn run(state: &mut State) -> bool {
    let viewport = (state.viewport_w, state.viewport_h);
    /* A page shown with no display list (a reload of the same document
     * while its sheets load) lays out whatever the fingerprint says. */
    let force = state.box_doc.is_none();
    let Some(dom) = state.page_dom.as_ref() else { return false };
    let print = super::print::print(state, dom);
    let (js, measured) = ((state.engine.is_some(), None), super::measure::measure(state));
    let page_css = &state.page_css;
    let mut css = || {
        let mut t = collect_css(dom);
        t.push_str(page_css);
        t
    };
    let text = CssCache::restyle(&mut state.css_cache, dom, &mut css, viewport, print, js);
    let Some(cache) = state.css_cache.as_mut() else { return false };
    let relaid = cache.relaid(print.0 ^ measured, viewport);
    let Some(s) = cache.styled().filter(|_| force || relaid || text.is_some()) else {
        return false;
    };
    if let Some(t) = text.as_deref() {
        super::fonts::queue_faces(
            &mut state.font_seen,
            &mut state.font_queue,
            state.base.as_ref(),
            t,
        );
    }
    /* A natural size is known once the raster, keyed by URL, decoded. */
    let (images, base) = (&state.images, state.base.as_ref());
    let natural = |src: &str| images.natural_for(base, src);
    let root = build(dom, &s.styles, &s.bg_images, &s.svg_paint, &s.grids, &s.pseudos, &natural);
    let doc = layout(&root, viewport);
    drop(root);
    /* The rectangles just produced are what a script gets when it
     * measures an element, so a read after a layout sees this one. */
    if let Some(dom) = state.page_dom.as_mut() {
        dom.record_rects(doc.frags.iter().map(|f| (f.node, f.x, f.y, f.w, f.h)));
    }
    state.box_doc = Some(doc);
    /* The layout's own unsized images and inline faces are its measures. */
    let measured = super::measure::measure(state);
    if let Some(c) = state.css_cache.as_mut() {
        c.relaid(print.0 ^ measured, viewport);
    }
    crate::browser::image::enqueue_from_doc(state);
    true
}
