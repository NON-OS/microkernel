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

//! The range each absolute axis reports over, read from the device's
//! configuration. A device without an absolute axis (a keyboard, a mouse)
//! answers size 0 and the axis is None.

use nonos_virtio::common::stable_read;
use nonos_virtio::Mmio;

use crate::constants::cfg::{ABS_MAX, ABS_MIN, CFG_ABS_INFO, CFG_LEN, SELECT, SIZE, SUBSEL};
use crate::constants::evdev::{ABS_X, ABS_Y};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Axis {
    pub min: i32,
    pub max: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Axes {
    pub x: Option<Axis>,
    pub y: Option<Axis>,
}

pub fn read(common: &Mmio, cfg: Mmio) -> Result<Axes, &'static str> {
    if cfg.len() < CFG_LEN {
        return Err("device configuration too short");
    }
    Ok(Axes { x: axis(common, cfg, ABS_X), y: axis(common, cfg, ABS_Y) })
}

/// Read under the configuration generation, so a change between the two
/// halves cannot pair a min with another axis's max.
fn axis(common: &Mmio, cfg: Mmio, code: u16) -> Option<Axis> {
    stable_read(common, || {
        cfg.w8(SELECT, CFG_ABS_INFO);
        cfg.w8(SUBSEL, code as u8);
        (cfg.r8(SIZE) >= 8)
            .then(|| Axis { min: cfg.r32(ABS_MIN) as i32, max: cfg.r32(ABS_MAX) as i32 })
    })
    .flatten()
}
