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

//! The switch back to the user area when firmware left a boot partition.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::EmmcResult;
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::super::card::Card;
use super::super::ext_csd::{PARTITION_ACCESS_MASK, PARTITION_CONFIG};
use super::super::ops::{say_failed, switch};

/// Reads and writes go to the user area. Firmware that booted from a boot
/// partition may leave PARTITION_ACCESS pointing there; it is switched back,
/// keeping the boot enable and boot ack bits as they are.
pub(super) fn user_area<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &mut Card,
) -> EmmcResult<()> {
    let Some(ext) = card.ext else {
        return Ok(());
    };
    if ext.partition_access() == 0 {
        return Ok(());
    }
    let value = ext.partition_config() & !PARTITION_ACCESS_MASK;
    h.say(
        Line::new()
            .s(b"partition access ")
            .dec(ext.partition_access() as u64)
            .s(b", switching to the user area"),
    );
    if let Err(e) = switch(h, card, PARTITION_CONFIG, value) {
        say_failed(h, b"CMD6 PARTITION_CONFIG", e);
        return Err(e);
    }
    if let Some(x) = card.ext.as_mut() {
        x.raw[PARTITION_CONFIG as usize] = value;
    }
    Ok(())
}
