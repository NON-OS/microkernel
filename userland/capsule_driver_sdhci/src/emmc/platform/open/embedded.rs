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

//! Refusing a generic host whose slot is not an embedded one.

use super::super::super::env::{Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::pci::Kind;
use super::super::super::sdhci::regs::{CAPABILITIES, SLOT_TYPE_EMBEDDED};
use super::super::super::sdhci::Caps;
use super::super::super::text::Line;
use super::super::discover::Found;
use super::super::env::{MmioWindow, SerialLog};

/// A generic SDHCI host is served only when its slot is embedded.
pub(super) fn check_embedded(log: &SerialLog, dev: &Found, io: &MmioWindow) -> EmmcResult<()> {
    if dev.kind == Kind::Generic {
        let caps = Caps { caps: io.r32(CAPABILITIES), caps1: 0, version: 0 };
        if caps.slot_type() != SLOT_TYPE_EMBEDDED {
            log.line(
                Line::new()
                    .s(b"slot type ")
                    .dec(caps.slot_type() as u64)
                    .s(b" is not embedded; skipped")
                    .as_bytes(),
            );
            return Err(EmmcError::NotEmbedded);
        }
    }
    Ok(())
}
