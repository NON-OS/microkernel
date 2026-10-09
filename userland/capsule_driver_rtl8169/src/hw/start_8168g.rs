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

use super::disable_rxdvgate;
use super::eri::{eri_modify, eri_write, ERIAR_MASK_0001, ERIAR_MASK_0011, ERIAR_MASK_1111};
use crate::chip::MacVersion;
use crate::regs::Regs;

/*
 * The steps every VER_40 to VER_52 start shares in Linux (rtl_hw_start_8168g,
 * _8168h_1, _8168ep, _8117): static FIFO sizes, the pause thresholds, a
 * packet filter reset, the RX data gate released, and ERI 0xC0 and 0xB8
 * cleared (the first two are rtl_set_fifo_size, the filter reset is
 * rtl_reset_packet_filter). The per-revision PCIe PHY (ephy) tables, ASPM latency and the
 * power-saving bits after them are firmware's tuning and are not repeated.
 * Until the gate is released no received frame reaches the ring.
 */
pub fn start_8168g(regs: &Regs, ver: MacVersion) -> Result<(), &'static str> {
    // rtl8168g_set_pause_thresholds: 0x38/0x48, or 0x2f/0x5f on the ep/8117.
    let (low, high) = if ver.0 >= 51 { (0x2F, 0x5F) } else { (0x38, 0x48) };
    // Every step runs, in Linux's order, even after one fails, as in Linux:
    // the gate must be released whatever an ERI write did.
    let steps = [
        eri_write(regs, 0xC8, ERIAR_MASK_1111, (0x08 << 16) | 0x02),
        eri_write(regs, 0xE8, ERIAR_MASK_1111, (0x10 << 16) | 0x06),
        eri_write(regs, 0xCC, ERIAR_MASK_0001, low),
        eri_write(regs, 0xD0, ERIAR_MASK_0001, high),
        eri_modify(regs, 0xDC, 0, 1 << 0),
        eri_modify(regs, 0xDC, 1 << 0, 0),
        {
            disable_rxdvgate(regs);
            Ok(())
        },
        eri_write(regs, 0xC0, ERIAR_MASK_0011, 0),
        eri_write(regs, 0xB8, ERIAR_MASK_0011, 0),
    ];
    steps.into_iter().find(|r| r.is_err()).unwrap_or(Ok(()))
}
