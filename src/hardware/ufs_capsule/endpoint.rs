// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::state::STATE;
use crate::hardware::block_device::wire::{Endpoint, Slot};

/// The kernel inbox the capsule replies to; its manifest's reply endpoint.
pub(super) const REPLY_INBOX: &str = "endpoint.4294967325";

static SLOT: Slot = Slot::new();

/// driver.ufs0 on the block wire.
pub static ENDPOINT: Endpoint = Endpoint {
    shown: "UFS (driver.ufs0)",
    sender: "kernel.driver_ufs",
    reply_inbox: REPLY_INBOX,
    state: &STATE,
    // The logical units REPORT LUNS names, as many as the capsule serves.
    units: 8,
    slot: &SLOT,
};
