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

//! The section-08 type scale and the space and radius ramps, in logical
//! pixels: `metrics()` reads each once at the display scale. Every size here
//! already clears the 17 px readable floor the toolkit clamps draw and measure
//! to, so a run measured at one of these sizes is the run that gets painted.

use nonos_toolkit::design::scale::scale;

const LABEL: f32 = 17.0;
const SECONDARY: f32 = 17.0;
const DATA: f32 = 18.0;
const ITEM: f32 = 19.0;
const BODY: f32 = 19.0;
const SECTION: f32 = 24.0;
const HERO: f32 = 34.0;
const PAGE: f32 = 40.0;
const DISPLAY: f32 = 52.0;

const S1: i32 = 4;
const S2: i32 = 6;
const S3: i32 = 9;
const S4: i32 = 14;
const S5: i32 = 18;
const S6: i32 = 22;
const S7: i32 = 28;

const R_WIN: u32 = 20;
const R_CARD: u32 = 14;
const R_COVER: u32 = 10;
const R_THUMB: u32 = 7;

/// The type, space and radius ramps at the display scale in force, in
/// drawing pixels.
pub struct Metrics {
    pub label: f32,
    pub secondary: f32,
    pub data: f32,
    pub item: f32,
    pub body: f32,
    pub section: f32,
    pub hero: f32,
    pub page: f32,
    pub display: f32,
    pub s1: i32,
    pub s2: i32,
    pub s3: i32,
    pub s4: i32,
    pub s5: i32,
    pub s6: i32,
    pub s7: i32,
    pub r_win: u32,
    pub r_card: u32,
    pub r_cover: u32,
    pub r_thumb: u32,
}

/// The ramps at the display scale in force: each logical size above read
/// once through `scale()`, so no caller scales one again.
pub fn metrics() -> Metrics {
    let s = scale();
    let (f, r) = (|v: f32| s.font(v), |v: u32| s.px(v));
    Metrics {
        label: f(LABEL),
        secondary: f(SECONDARY),
        data: f(DATA),
        item: f(ITEM),
        body: f(BODY),
        section: f(SECTION),
        hero: f(HERO),
        page: f(PAGE),
        display: f(DISPLAY),
        s1: px(S1),
        s2: px(S2),
        s3: px(S3),
        s4: px(S4),
        s5: px(S5),
        s6: px(S6),
        s7: px(S7),
        r_win: r(R_WIN),
        r_card: r(R_CARD),
        r_cover: r(R_COVER),
        r_thumb: r(R_THUMB),
    }
}

/// `logical` pixels at the display scale in force, in drawing pixels, signed
/// for the player's boxes.
pub fn px(logical: i32) -> i32 {
    scale().px(logical.max(0) as u32) as i32
}

/// The scale's hairline, in drawing pixels.
pub fn hair() -> i32 {
    scale().hair() as i32
}

pub fn line_h(px: f32) -> i32 {
    (px * 1.45) as i32
}

pub fn cap_h(px: f32) -> i32 {
    (px * 0.72) as i32
}

pub fn pill(h: i32) -> u32 {
    (h.max(0) / 2) as u32
}
