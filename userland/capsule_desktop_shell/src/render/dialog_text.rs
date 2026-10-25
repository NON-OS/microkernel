// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A dialog's text: one line cut to its width with an ellipsis, and where
//! a paragraph wraps, at a space when there is one in reach.

use crate::render::measure_aa::{measure_aa, truncate_to_width};
use crate::render::text_aa::text_aa;
use crate::state::Context;

const ELLIPSIS: &str = "...";

pub(super) fn line(ctx: &Context, x: u32, top_y: u32, text: &str, argb: u32, px: f32, max_w: u32) {
    let fitted = truncate_to_width(text, px, max_w);
    if fitted.len() == text.len() {
        text_aa(ctx, x, top_y, text, argb, px);
        return;
    }
    let room = max_w.saturating_sub(measure_aa(ELLIPSIS, px));
    let cut = truncate_to_width(text, px, room);
    let advance = text_aa(ctx, x, top_y, cut, argb, px);
    text_aa(ctx, x + advance, top_y, ELLIPSIS, argb, px);
}

pub(super) fn wrap_at(text: &str, px: f32, max_w: u32) -> usize {
    let fitted = truncate_to_width(text, px, max_w);
    if fitted.len() == text.len() {
        return text.len();
    }
    match fitted.rfind(' ') {
        Some(i) if i > 0 => i,
        _ if !fitted.is_empty() => fitted.len(),
        _ => text.char_indices().nth(1).map_or(text.len(), |(i, _)| i),
    }
}
