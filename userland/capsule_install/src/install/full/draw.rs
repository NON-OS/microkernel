/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* A frame of the full-screen installer, and the request ids it is sent under. */

use super::surface::Surface;
use crate::install::state::{Screen, State};
use crate::install::ui::full::paint;

/* A disk is being written or read back. */
pub(super) fn busy(state: &State) -> bool {
    matches!(state.screen, Screen::Writing | Screen::Verifying)
}

pub(super) fn draw(state: &mut State, s: &Surface, rid: &mut u32) {
    paint(state, &mut s.buffer());
    s.commit(next(rid));
}

pub(super) fn next(rid: &mut u32) -> u32 {
    *rid = rid.wrapping_add(1).max(4);
    *rid
}
