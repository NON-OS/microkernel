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

//! Who may hold the two capabilities that reach past a capsule's own work,
//! stated as what is true on every image rather than as a wish (U3).
//!
//! Admin: the kernel asks for it on reboot, shutdown, the policy push and
//! every ACPI call, so init, the policy service, the ACPI and power
//! capsules and the two installers hold it by their signed manifests. The
//! claim names them, and any other holder is a red.
//!
//! Debug writes to the serial console. A standard image lets capsules use
//! it (`capsule-serial-debug`) and the Linux personality holds it on every
//! image, so "nothing can write a log" is false there. What must hold is
//! narrower: nothing that takes keys or input, or holds a secret, can log;
//! the kernel withholds Debug from such a capsule off the dev profile
//! (`secret_log`), and this check reads whether it did.

pub const DEBUG: u64 = 1 << 8;
pub const ADMIN: u64 = 1 << 9;
pub const INPUT_SOURCE: u64 = 1 << 21;
pub const KEYRING: u64 = 1 << 28;
pub const DEVICE_SECRET: u64 = 1 << 35;
pub const INPUT_CONSUMER: u64 = 1 << 36;
/// What a logging capsule must not also see.
pub const SECRETS: u64 = INPUT_SOURCE | KEYRING | DEVICE_SECRET | INPUT_CONSUMER;

/// The process names the kernel gives the Admin holders.
pub const ADMIN_HOLDERS: &[&[u8]] = &[b"init", b"policy", b"acpi", b"power"];

pub const ADMIN_CLAIM: &[u8] = b"admin: only init, policy, acpi, power";
pub const LOG_CLAIM: &[u8] = b"nothing holding keys, input or secrets can log";

/// Holders of `mask` whose name is not in `named`.
pub fn beyond<'a>(caps: impl Iterator<Item = (u64, &'a [u8])>, mask: u64, named: &[&[u8]]) -> u32 {
    caps.filter(|(m, n)| m & mask != 0 && !named.contains(n)).count() as u32
}

/// Capsules that can log and also see keys, input or a secret.
pub fn secret_loggers(caps: impl Iterator<Item = u64>) -> u32 {
    caps.filter(|m| m & DEBUG != 0 && m & SECRETS != 0).count() as u32
}
