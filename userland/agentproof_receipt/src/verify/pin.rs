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

//! Trust on first use: the trust file a receipt would verify under, taken
//! from that receipt. Its AK, its release's PCR 8, 9 and 14, and the
//! supervisor that sealed its record are pinned as they are, and the trust
//! is given only once the receipt verifies under it, so every check but
//! "this AK, release and supervisor are ones I trust" has passed. That one
//! is the pin's: nothing here says the TPM's maker, or anyone, vouched for
//! them (docs/agentproof/receipt.md).

use alloc::string::String;
use alloc::vec;

use nonos_proof_abi::decode_record;
use nsh_attest::Doc;

use super::{verify, Refusal};
use crate::napr::read::parse;
use crate::trust::{Release, Supervisor, Trust};

/// The release name a pinned trust file gives the release it pinned.
pub const FIRST_USE: &str = "first-use";

pub fn pin(bytes: &[u8]) -> Result<Trust, Refusal> {
    let r = parse(bytes).map_err(Refusal::Format)?;
    let doc = Doc::parse(r.doc).map_err(Refusal::Doc)?;
    let pcr = |i: u8| doc.pcr(i).copied().ok_or(Refusal::Release);
    let record = decode_record(r.record).ok_or(Refusal::Record)?;
    let trust = Trust {
        releases: vec![Release {
            version: String::from(FIRST_USE),
            pcr8: pcr(8)?,
            pcr9: pcr(9)?,
            pcr14: pcr(14)?,
        }],
        supervisors: vec![Supervisor { meas: record.sup_meas, caps: record.sup_caps }],
        ak: *doc.ak_public,
    };
    verify(bytes, &trust)?;
    Ok(trust)
}
