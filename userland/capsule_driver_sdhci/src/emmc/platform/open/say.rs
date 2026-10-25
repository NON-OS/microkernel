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

//! Console lines about the host being opened and a refused DMA region.

use super::super::super::env::Log;
use super::super::super::error::EmmcError;
use super::super::super::pci::Kind;
use super::super::super::text::Line;
use super::super::discover::Found;
use super::super::env::SerialLog;

pub(super) fn say_host(log: &SerialLog, dev: &Found) {
    log.line(
        Line::new()
            .s(b"host ")
            .hex(dev.vendor as u64)
            .s(b":")
            .hex(dev.device as u64)
            .s(match dev.kind {
                Kind::IntelEmmc => b" (Intel eMMC)" as &[u8],
                Kind::Generic => b" (SDHCI)",
            })
            .as_bytes(),
    );
}

pub(super) fn say_dma(log: &SerialLog, e: EmmcError) {
    if let EmmcError::Broker(r) = e {
        log.line(Line::new().s(b"DMA region refused (").dec(r.unsigned_abs()).s(b")").as_bytes());
    }
}
