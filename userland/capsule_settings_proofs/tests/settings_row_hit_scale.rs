// NONOS Operating System (AGPL-3.0-or-later)
//! At 1x, 1.25x, 1.5x and 2x each editable row's label takes a click as the
//! row and the switch drawn in its control column takes one as the control,
//! on every page, in a window as wide as the panel opens at the scale. Its own
//! binary, so setting the scale races no other test.

use capsule_settings_proofs::settings::manifest::WIDTH;
use capsule_settings_proofs::settings::schema::blocks_for;
use capsule_settings_proofs::settings::schema::rows::Row;
use capsule_settings_proofs::settings::section::{Section, SECTIONS, SECTION_COUNT};
use capsule_settings_proofs::settings::state::{State, Wifi};
use capsule_settings_proofs::settings::ui::control_geom::switch_rect;
use capsule_settings_proofs::settings::ui::hover::target_at;
use capsule_settings_proofs::settings::ui::metrics::metrics;
use capsule_settings_proofs::settings::ui::target::Target;
use capsule_settings_proofs::settings::ui::walk::{walk, Item};
use nonos_toolkit::design::scale::{scale, set_scale};

const VIEW_H: u32 = 100_000;

/// Section `section`'s page in a window as wide as the panel opens at the scale.
fn page(section: Section) -> State {
    let (wifi, hover, win_w) = (Wifi { saved_count: 0 }, Target::None, scale().px(WIDTH));
    let (scroll_px, search_scroll, wifi_network_count) = ([0; SECTION_COUNT], 0, 0);
    State { section, scroll_px, search_scroll, win_w, wifi_network_count, wifi, hover }
}

#[test]
fn a_row_click_lands_on_the_label_or_control_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let mut seen = 0;
        for s in SECTIONS {
            seen += rows(&page(s), q);
        }
        assert!(seen > 0, "q={q}: no editable row was checked");
    }
    set_scale(4);
}

/// Checks every editable row on `state`'s page and says how many there were.
fn rows(state: &State, q: u32) -> usize {
    let m = metrics();
    let (card_x, side) = (m.pane_pad_x, m.sidebar_w);
    let card_w = state.win_w - side - card_x * 2;
    let blocks = blocks_for(state.section);
    let mut index = 0;
    walk(state, |y, h, item| {
        let editable = match item {
            Item::Row(bi, ri) => matches!(blocks[bi].rows[ri], Row::Field(_)),
            Item::Wallpaper(_) => true,
            _ => false,
        };
        if !editable {
            return;
        }
        let under = |x: u32, y: u32| target_at(state, (side + x) as i32, y as i32, VIEW_H, None);
        let label = card_x + m.card_pad_x;
        for ry in [y, y + h - 1] {
            assert!(under(label, ry) == Target::Row(index), "q={q}: row {index}'s label");
        }
        let (sx, sy) = switch_rect(card_x, card_w, y as i32, h);
        let corners = [(sx, sy as u32), (sx + m.switch_w - 1, sy as u32 + m.switch_h - 1)];
        for (cx, cy) in corners {
            assert!(under(cx, cy) == Target::Control(index), "q={q}: row {index}'s switch");
        }
        index += 1;
    });
    index
}
