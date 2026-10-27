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

//! A key for a kernel label under the device secret's policy (mod.rs): a
//! kernel the release approved, on the firmware and bootloader this machine
//! has, computes it, so a signed update keeps it where PCR 9 alone would
//! not. The data volume's key is wrapped under one
//! (fs/blockfs_volume/key_pick.rs).
//!
//! The handover's key is the same without the bootloader: a policy over
//! another set of PCRs makes another primary, so the two never coincide. The
//! volume key is wrapped under one only for the first boot of the next
//! release's bootloader (fs/blockfs_volume/handover.rs).

use alloc::vec::Vec;

use super::approval::Approval;
use super::consts::{HANDOVER_PCRS, MACHINE_PCRS};
use super::derive::with_approved_key;
use crate::security::tpm::machine_key::consts::DIGEST_LEN;
use crate::security::tpm::machine_key::hmac::{build_hmac, parse_hmac};
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::KeyError;

/// The key for kernel label `label` on this machine's firmware, bootloader
/// and Secure Boot state.
pub fn approved_key(a: &Approval, label: &[u8]) -> Result<[u8; DIGEST_LEN], KeyError> {
    key_under(a, &MACHINE_PCRS, label)
}

/// The key for kernel label `label` on this machine's firmware and Secure
/// Boot state, whatever bootloader measured the kernel.
pub fn handover_key(a: &Approval, label: &[u8]) -> Result<[u8; DIGEST_LEN], KeyError> {
    key_under(a, &HANDOVER_PCRS, label)
}

/// A leading zero byte marks the label the kernel's, as the machine key's
/// kernel labels are (kernel_label.rs), and no caller reaches this policy's
/// key with a label of its own.
fn key_under(a: &Approval, machine: &[u8], label: &[u8]) -> Result<[u8; DIGEST_LEN], KeyError> {
    let mut kernel = Vec::with_capacity(label.len() + 1);
    kernel.push(0);
    kernel.extend_from_slice(label);
    let hmac = |key, session| parse_hmac(&run(&build_hmac(key, session, &kernel))?);
    with_approved_key(a, machine, hmac)
}
