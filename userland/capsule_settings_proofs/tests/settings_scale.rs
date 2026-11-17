// NONOS Operating System (AGPL-3.0-or-later)
//! The Settings window keeps logical metrics and draws them at the display
//! scale: at 1.25x, 1.5x and 2x every size is its 1x size through `scale().px`
//! or `scale().font`, and a window grown by the same scale holds the sidebar,
//! the page head and each row's control. The head band holds its title and
//! subtitle at every text size. Its own binary, so setting the scale races no
//! other test.

use capsule_settings_proofs::settings::manifest::{HEIGHT, WIDTH};
use capsule_settings_proofs::settings::section::SECTIONS;
use capsule_settings_proofs::settings::ui::control_geom::{right_edge, slider_x, switch_rect};
use capsule_settings_proofs::settings::ui::metrics::metrics;
use capsule_settings_proofs::settings::ui::nav_geom::{row_w, row_x, row_y};
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::font::ttf::{line_height, text_scale};

/// Every length, then every font size, the panel lays out with.
#[rustfmt::skip]
fn sizes() -> (Vec<u32>, Vec<f32>) {
    let m = metrics();
    let lengths = [
        m.sidebar_w, m.nav_top, m.nav_h, m.nav_gap, m.nav_pad_x, m.pane_pad_x, m.pane_pad_top,
        m.head_icon, m.head_icon_gap, m.head_line_gap, m.head_h, m.card_gap, m.card_pad_x,
        m.card_head_h, m.card_head_note_h, m.row_h, m.row_note_h, m.switch_w, m.switch_h,
        m.slider_w, m.pill_h,
    ];
    (lengths.to_vec(), vec![m.title_px, m.subtitle_px, m.body_px, m.note_px])
}

/// How tall the page head's title and subtitle stand, as the painter steps them.
fn head_text() -> u32 {
    let m = metrics();
    let lines = line_height(m.title_px) + line_height(m.subtitle_px);
    lines as u32 + m.head_line_gap
}

#[test]
fn every_size_grows_with_the_scale_and_the_window_still_fits() {
    set_scale(4);
    let (lengths, fonts) = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (s, m) = (scale(), metrics());
        let px = lengths.iter().map(|&n| s.px(n)).collect();
        let want = (px, fonts.iter().map(|&f| s.font(f)).collect());
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        let (w, h) = (s.px(WIDTH), s.px(HEIGHT));
        let last = row_y(SECTIONS.len() - 1) + m.nav_h;
        assert!(last <= h, "q={q}: the sidebar's last entry leaves the window");
        assert!(row_x() + row_w() <= m.sidebar_w, "q={q}: a nav entry leaves the sidebar");
        let (card_x, pane_w) = (m.pane_pad_x, w - m.sidebar_w);
        let card_w = pane_w - card_x * 2;
        let label = card_x + m.card_pad_x;
        assert!(label < slider_x(card_x, card_w), "q={q}: no room left for a label");
        assert!(right_edge(card_x, card_w) <= card_x + card_w, "q={q}: control leaves card");
        let (sx, sy) = switch_rect(card_x, card_w, 0, m.row_h);
        assert!(sx >= slider_x(card_x, card_w) && sy >= 0, "q={q}: switch leaves its column");
        assert!(sy as u32 + m.switch_h <= m.row_h, "q={q}: switch leaves its row");
        assert!(m.pill_h <= m.card_head_h, "q={q}: a card's pill leaves its head");
        let fixed = lengths[10];
        assert!(q < 8 || head_text() > fixed, "q={q}: a 1x band would have held the head");
        for steps in 0..5 {
            text_scale::set_steps(steps);
            let what = format!("q={q} text size {steps}");
            let head = metrics().head_h;
            assert!(head_text() <= head && m.head_icon <= head, "{what}: head overflows");
        }
        text_scale::set_steps(1);
    }
    set_scale(4);
}
