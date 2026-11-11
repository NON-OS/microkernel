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

//! Each About screen reports the height it paints: at 1x, 1.25x, 1.5x and 2x,
//! in a window grown by the scale, its last painted row is its extent's last,
//! so the wheel and End stop at its foot. Its own binary, so setting the scale
//! races no other test.

use apps_proofs::about::section::{Section, SECTIONS};
use apps_proofs::about::state::State;
use apps_proofs::about::ui::chrome::pane_rect;
use apps_proofs::about::ui::metrics::{WIN_H, WIN_W};
use apps_proofs::about::ui::screens::{display, licenses, overview, proofs, system, trust, verify};
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::Rect;

/// The height the section reports, before the frame lifts it to the pane's.
fn extent(s: &State, r: &Rect) -> u32 {
    match s.section {
        Section::Overview => overview::content_h(r),
        Section::Proofs => proofs::content_h(s, r),
        Section::System => system::content_h(r),
        Section::Trust => trust::content_h(s, r),
        Section::Verify => verify::content_h(r),
        Section::Display => display::content_h(r),
        Section::Licenses => licenses::content_h(r),
    }
}

/// How far down the section paints, scrolled so its foot lands mid-view.
fn painted(s: &mut State, r: &Rect, view: u32) -> u32 {
    s.scroll = extent(s, r).saturating_sub(view / 2);
    let (mut px, at) = (vec![0u32; (r.w * view) as usize], Rect { x: 0, y: 0, w: r.w, h: view });
    let fb = &mut PaintBuffer { pixels: &mut px, stride_words: r.w, width: r.w, height: view };
    match s.section {
        Section::Overview => overview::paint(s, fb, &at),
        Section::Proofs => proofs::paint(s, fb, &at),
        Section::System => system::paint(s, fb, &at),
        Section::Trust => trust::paint(s, fb, &at),
        Section::Verify => verify::paint(s, fb, &at),
        Section::Display => display::paint(s, fb, &at),
        Section::Licenses => licenses::paint(s, fb, &at),
    }
    let last = px.chunks(r.w as usize).rposition(|row| row.iter().any(|&p| p != 0));
    s.scroll + last.map_or(0, |y| y as u32 + 1)
}

#[test]
fn every_screen_reports_the_height_it_paints_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (r, view) = (pane_rect(scale().px(WIN_W), scale().px(WIN_H)), scale().px(WIN_H));
        for section in SECTIONS {
            let mut s = State::new();
            s.select(section);
            assert_eq!(painted(&mut s, &r, view), extent(&s, &r), "q={q} {}", section.index());
        }
    }
    set_scale(4);
}
