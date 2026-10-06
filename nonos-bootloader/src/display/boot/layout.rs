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

//! Splash geometry: the shared scene, the emblem on the left, and on the
//! right the platform facts, then a headline saying what the loader is doing,
//! the latest log line under it, a bar, and the step list. The proofs panel
//! later takes the area from the headline down.

use super::proofs::panel_height;
use crate::display::ink::{metrics, scene, Scene, Style};

/// Rows of the step list: every stage after init.
pub const STEPS: u32 = 10;

#[derive(Clone, Copy)]
pub struct Splash {
    pub s: Scene,
    pub u: u32,
    pub col_x: u32,
    pub col_w: u32,
    /// The facts row: Secure Boot, TPM, the kernel's check.
    pub chips_y: u32,
    /// The headline, and the latest log line under it.
    pub title_y: u32,
    pub detail_y: u32,
    /// The bar of steps done.
    pub bar_y: u32,
    /// The step list: its first row and each row's height.
    pub list_y: u32,
    pub row_h: u32,
    /// The area the proofs panel takes: from the headline down.
    pub panel_y: u32,
    pub panel_h: u32,
    pub footer_y: u32,
}

pub fn splash() -> Splash {
    let (mono, body, head) = (metrics(Style::Mono), metrics(Style::Body), metrics(Style::Heading));
    let u = scene(0).u;
    let facts_h = mono.line * 2 + 4 * u;
    let row_h = body.line + 2 * u;
    let top = head.line + u + mono.line + 3 * u + 2 + 4 * u;
    let panel_h = (top + STEPS * row_h).max(panel_height(u));
    let s = scene(facts_h + 6 * u + panel_h);
    let panel_y = s.col_y + facts_h + 6 * u;
    let detail_y = panel_y + head.line + u;
    let bar_y = detail_y + mono.line + 3 * u;
    Splash {
        s,
        u,
        col_x: s.col_x,
        col_w: s.col_w,
        chips_y: s.col_y,
        title_y: panel_y,
        detail_y,
        bar_y,
        list_y: bar_y + 2 + 4 * u,
        row_h,
        panel_y,
        panel_h,
        footer_y: s.footer_y,
    }
}
