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

//! The boot screen's proofs panel, fed from what this boot verified and the
//! evidence it gathered for the kernel.

use nonos_boot::display::{show_proofs, Proofs};
use nonos_boot::kernel_verify::CryptoVerifyResult;
use nonos_boot::security::SecurityContext;
use uefi::prelude::*;

use super::boot_evidence::BootEvidence;

pub fn show(
    st: &SystemTable<Boot>,
    gop: bool,
    crypto: &CryptoVerifyResult,
    security: &SecurityContext,
    evidence: &BootEvidence,
) {
    let proofs = Proofs {
        crypto,
        security,
        tcg_log: evidence.log,
        trailer: evidence.trailer,
        record: evidence.record,
    };
    show_proofs(st.boot_services(), gop, &proofs);
}
