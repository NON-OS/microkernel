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

//! The device secret the anonymous attestation proves with, derived by the TPM
//! and usable only on an approved chain.
//!
//! Like the machine key it is never stored: it is an HMAC under a primary key
//! the TPM rebuilds from its owner seed and a template, and the template's
//! authorization policy is what decides who can use it. That policy is
//!
//!   PolicyAuthorize(release key, ref) over the kernel's PCR 9,
//!   then PolicyPCR(0, 4, 7) as they are on this machine.
//!
//! PCR 9 is the kernel and its trailer, the same on every machine for one
//! release, so the release signs it: a signed kernel update keeps the secret.
//! PCR 0, 4 and 7 are firmware, the bootloader as the firmware measured it, and
//! the Secure Boot state. They differ per machine and cannot be signed once
//! for all, so they are bound as they are: a firmware or bootloader change is a
//! new secret and a re-enrollment, which the registrar treats as replacement.
//!
//! What this gives the proof: only a chain whose kernel the release approved,
//! on the firmware and bootloader this device enrolled under, can compute `s`.
//! The proof shows the holder of `s` opened an approved chain; this is what
//! shows the machine booted it.

mod approval;
mod command;
mod consts;
mod derive;
mod digest;
mod draw;
mod public;
mod verify;

pub use approval::{from_boot, Approval, POLICY_REF};
pub use derive::device_secret;
pub use public::{p256_name, p256_public};
