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

//! The kernel's lines, read off the result kernel_verify returned.

use super::rows::Row;
use super::state::State;
use crate::display::text::Text;
use crate::kernel_verify::CryptoVerifyResult;

/// The v4 trailer: its path and its STARK pass or fail together. The root is
/// the one the path folded to, which `attest_policy` gives only once it has.
pub fn trailer(c: &CryptoVerifyResult) -> Row {
    let t = Text::new();
    let (state, detail) = if !c.proof_present {
        (State::Absent, t.push(b"no v4 trailer in a valid footer"))
    } else if c.path_attested {
        let root = c.attest_policy().kernel_root;
        let t = t.push(b"STARK + path, proof ").size(c.proof_len);
        (State::Verified, t.push(b", root ").hex(&root[..4]))
    } else if c.proof_len == 0 {
        (State::Failed, t.push(b"not a v4 kernel trailer"))
    } else {
        (State::Failed, t.push(b"STARK or path did not verify, proof ").size(c.proof_len))
    };
    Row { label: b"KERNEL", state, detail }
}

/// The release signatures. Validation admits only an image signed with both
/// Ed25519 and ML-DSA-65, and `signature_valid` holds only when both verified.
pub fn signature(c: &CryptoVerifyResult) -> Row {
    let (state, detail): (State, &[u8]) = match (c.signature_present, c.signature_valid) {
        (false, _) => (State::Absent, b"no valid signed footer"),
        (true, true) => (State::Verified, b"Ed25519 + ML-DSA-65"),
        (true, false) => (State::Failed, b"Ed25519 + ML-DSA-65 did not both verify"),
    };
    Row { label: b"KERNEL SIGNATURE", state, detail: Text::new().push(detail) }
}
