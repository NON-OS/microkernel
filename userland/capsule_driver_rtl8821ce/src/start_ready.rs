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

//! The rest of start-up on a powered MAC: the firmware, the transmit and
//! receive engines, and the MAC tables.

use crate::serve::Stage;
use crate::setup::Mapped;
use crate::{fwload, mac, setup, status};

/// Load the firmware and bring the MAC up on `mapped`, reporting how far it
/// got.
pub fn up_to_ready(mapped: Mapped) -> (Option<Mapped>, Stage) {
    // With the MAC powered, load the 8051 firmware. On real silicon this runs the
    // reserved-page staging and DDMA; off-silicon the whole path bar the DMA
    // mapping and the card's own execution is proven in rtl8821ce_proofs.
    if fwload::load(&mapped.regs, mapped.device_id, mapped.claim_epoch).is_err() {
        status::line(b"[rtl8821ce] firmware download failed\n");
        setup::stopped(&mapped);
        return (Some(mapped), Stage::FirmwareFailed);
    }
    status::line(b"[rtl8821ce] firmware loaded and ready\n");
    // Switch the transmit and receive engines on: queue mapping, the reserved
    // page boundaries and link-list table, and the H2C queue. Without this the
    // register table below configures the receiver but it never runs, so the
    // radio comes up yet hears nothing.
    if !mac::init_trx_cfg(&mapped.regs) {
        status::line(b"[rtl8821ce] trx engine enable failed\n");
        setup::stopped(&mapped);
        return (Some(mapped), Stage::FirmwareFailed);
    }
    status::line(b"[rtl8821ce] trx engines enabled\n");
    // Register work only: protocol, EDCA, beacon and WMAC configuration.
    mac::run_mac_table(&mapped.regs, mac::MAC_INIT);
    status::line(b"[rtl8821ce] mac initialised\n");
    (Some(mapped), Stage::Ready)
}
