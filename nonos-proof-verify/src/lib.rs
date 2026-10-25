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

//! The NONOS proof verifier. A kernel, bootloader or capsule ships a v4
//! trailer: the path from its slot to a policy root and a STARK proof of the
//! same slot. This crate checks one the way the gates do, with the gates' own
//! path crate (`nonos-attest-path`) and STARK verifier (`nox_verify`), over a
//! context it builds itself from the image, so anyone can repeat the boot's
//! and the spawn's decision off the machine.

mod check;
#[cfg(feature = "release-roots")]
mod cli;
mod measure;
#[cfg(feature = "release-roots")]
mod report;
#[cfg(feature = "release-roots")]
mod roots;
#[cfg(feature = "release-roots")]
mod run;
mod slot;
mod transcript;

pub use check::{check_v4, statement_kind, Proof, Refused, DEPTH};
#[cfg(feature = "release-roots")]
pub use cli::{cli, parse as parse_args, Output, HELP};
pub use measure::{authenticode_of, blake3_of, Slot, BOOT_EPOCH, POLICY_EPOCH};
#[cfg(feature = "release-roots")]
pub use report::{kind_name, q as json_string, shown, Failure, Outcome, Report, Subject, Verdict};
#[cfg(feature = "release-roots")]
pub use report::{FORMAT, FORMAT_VERSION};
#[cfg(feature = "release-roots")]
pub use roots::{release_of, Release, RELEASE};
#[cfg(feature = "release-roots")]
pub use run::{against, in_slot, read_file, read_file_within, run, run_with};
#[cfg(feature = "release-roots")]
pub use run::{Mode, Reader, Request, MAX_FILE};
pub use slot::{opens, Opens};
pub use transcript::{hex32, read as read_transcript, rebuild, Transcript, HEADER};

/// Bytes as lowercase hex.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
