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

//! Bytes as the window shows them.

use alloc::string::String;

use nonos_device_attest::Error;

/// Lower-case hex.
pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| alloc::format!("{x:02x}")).collect()
}

/// A verifier id, which the request held to printable ASCII.
pub fn ascii(b: &[u8]) -> String {
    b.iter().map(|&c| char::from(c)).collect()
}

/// Why the prover or the check of its proof gave nothing to save.
pub fn prover(e: &Error) -> String {
    match e {
        Error::Shape => "the statement does not fit the circuit".into(),
        Error::Witness(why) => alloc::format!("nothing proven: {why}"),
        Error::Entropy => "too little entropy to hide the secret".into(),
        Error::Prover => "the prover produced nothing".into(),
        Error::NotVerified(why) => alloc::format!("the proof does not verify: {why}"),
        Error::Rank(_) => "its zero-knowledge check failed: prove again".into(),
    }
}
