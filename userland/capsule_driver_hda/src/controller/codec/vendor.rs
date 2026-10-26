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
//! Each vendor's setup for every codec of a kind: Realtek's coefficients
//! (`realtek`), Cirrus Logic's init sequences (`cirrus`) and IDT's port
//! power map (`idt`). Conexant's family-wide setup is EAPD on every pin
//! that has it, which `external` gives every codec; its other setup in
//! Linux (conexant.c) is per machine. Each step leaves a codec of another
//! vendor alone.

use super::widget::Codec;
use super::{cirrus, idt, realtek};
use crate::controller::verb::Link;
use crate::error::HdaResult;

/// Every vendor step for `codec`; `hp_pin` and `hp_plugged` as `realtek::init`.
pub fn init(link: &mut Link, codec: &Codec, hp_pin: u8, hp_plugged: bool) -> HdaResult<()> {
    realtek::init(link, codec, hp_pin, hp_plugged)?;
    cirrus::init(link, codec)?;
    idt::init(link, codec)
}
