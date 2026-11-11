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

//! Every size the audio player lays out with is its 1x size at the display
//! scale, and a 1440 by 900 window grown by the same scale still holds every
//! part it lays out.

use apps_proofs::audio_ui::geometry::{page, shell};
use apps_proofs::audio_ui::metrics::metrics;
use apps_proofs::audio_ui::screen::{downloads_geom, home_geom, lib_geom, search_geom, settings};
use apps_proofs::audio_ui::shell::{bar, rail_geom};
use nonos_toolkit::design::scale::{scale, set_scale};

#[rustfmt::skip]
fn sizes() -> (Vec<u32>, Vec<f32>) {
    let (m, sh) = (metrics(), shell(scale().px(1440), scale().px(900)));
    let b = bar::bar(sh.transport);
    let signed = [
        m.s1, m.s2, m.s3, m.s4, m.s5, m.s6, m.s7, lib_geom::row_h(), rail_geom::qrow_h(),
        home_geom::hero_h(), bar::play_r(), settings::fact_h(), sh.sidebar.w, sh.rail.w,
        sh.topbar.h, sh.transport.h, b.thumb.w, b.volume.w, b.volume.h, b.speaker.w,
    ];
    let radii = [m.r_win, m.r_card, m.r_cover, m.r_thumb];
    let lengths = signed.iter().map(|&n| n as u32).chain(radii).collect();
    let fonts = [m.label, m.secondary, m.data, m.item, m.body, m.section, m.hero, m.page, m.display];
    (lengths, fonts.to_vec())
}

#[test]
fn every_size_grows_with_the_scale_and_the_window_still_holds_it() {
    set_scale(4);
    let (lengths, fonts) = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let s = scale();
        let want = (
            lengths.iter().map(|&n| s.px(n)).collect(),
            fonts.iter().map(|&f| s.font(f)).collect(),
        );
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        let (w, h) = (s.px(1440) as i32, s.px(900) as i32);
        let sh = shell(w as u32, h as u32);
        assert_eq!(sh.sidebar.w + sh.topbar.w + sh.rail.w, w, "q={q}: the columns fill the window");
        assert!(sh.rail.w > 0 && sh.content.h > 0, "q={q}: the rail folds at the scaled window");
        assert!(shell(s.px(1000), s.px(700)).rail.w == 0, "q={q}: a narrow window keeps the rail");
        let p = page(&sh);
        assert!(
            p.x >= sh.sidebar.w && p.right() <= sh.rail.x && p.bottom() <= sh.transport.y,
            "q={q}"
        );
        assert!(rail_geom::queue_visible(sh.rail) >= 3, "q={q}: the rail shows too few rows");
        assert!(rail_geom::reserved() <= rail_geom::body(sh.rail).h, "q={q}: the rail overflows");
        assert!(lib_geom::visible(p) >= 5 && search_geom::visible(p) >= 5, "q={q}: lists");
        assert!(downloads_geom::visible(p) >= 3 && home_geom::shelves(p) >= 1, "q={q}: pages");
        let panel = settings::panel_rect(p);
        let last = settings::toggle_row(p, 3);
        assert!(last.bottom() <= panel.bottom() && last.h == s.px(52) as i32, "q={q}: toggles");
        assert!(settings::meter_rect(p).bottom() <= p.bottom(), "q={q}: the meter leaves");
        assert!(settings::tiles_rect(p).bottom() <= p.bottom(), "q={q}: the tiles leave");
    }
    set_scale(4);
}
