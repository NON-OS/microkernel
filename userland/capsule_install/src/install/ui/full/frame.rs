/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * A full-screen frame: setup's panel, then on the right the title, one line
 * saying what the screen is for, the screen's own body, and the keys.
 */

use nonos_app_skeleton::PaintBuffer;

use super::chrome::{panel, panel_w};
use super::palette::{CARD_BG, FG, HINT, RULE};
use super::subtitle::subtitle;
use crate::install::state::{Screen, State};
use crate::install::ui::frame::{screen, Body};
use crate::install::ui::hints::hints;
use crate::install::ui::metrics::SMALL_PX;
use crate::install::ui::text::{line, right};
use crate::install::ui::theme::WARN;

const TITLE_PX: f32 = 28.0;
const SUB_PX: f32 = 16.0;
const BODY_TOP: u32 = 124;
const FOOT_H: u32 = 64;

pub fn paint(state: &mut State, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    fb.clear(CARD_BG);
    panel(fb, state.screen);
    let x = panel_w(w) + 32;
    line(fb, x, 36, state.screen.title(), FG, TITLE_PX);
    line(fb, x, 80, subtitle(state.screen), HINT, SUB_PX);
    let bw = w.saturating_sub(x + 48);
    let bh = h.saturating_sub(BODY_TOP + FOOT_H + 16);
    screen(state, fb, Body { x, y: BODY_TOP, w: bw, h: bh });
    let foot = h.saturating_sub(FOOT_H);
    fb.fill_rect(x, foot, bw, 1, RULE);
    let (left, keys) = hints(state);
    let busy = matches!(state.screen, Screen::Writing | Screen::Verifying);
    line(fb, x, foot + 22, left, HINT, SMALL_PX);
    right(fb, x + bw, foot + 22, keys, if busy { WARN } else { FG }, SMALL_PX);
}
