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

use alloc::vec::Vec;

use super::approval::Approval;
use super::derive::with_approved_key;
use crate::security::tpm::machine_key::consts::DIGEST_LEN;
use crate::security::tpm::machine_key::hmac::{build_hmac, parse_hmac};
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::KeyError;

/// The key for kernel label `label`. A leading zero byte marks it the
/// kernel's, as the machine key's kernel labels are (kernel_label.rs), and no
/// caller reaches this policy's key with a label of its own.
pub fn approved_key(a: &Approval, label: &[u8]) -> Result<[u8; DIGEST_LEN], KeyError> {
    let mut kernel = Vec::with_capacity(label.len() + 1);
    kernel.push(0);
    kernel.extend_from_slice(label);
    with_approved_key(a, |key, session| parse_hmac(&run(&build_hmac(key, session, &kernel))?))
}
