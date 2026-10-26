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

//! What the pane says when a proof does not come (I-B9a, section 4). The
//! codes are nonos_proof_abi::wire's, held to it by proof_call.rs.
//!
//! Pure; market_proofs holds it.

use alloc::vec::Vec;

use super::proof_fail::Fail;

pub const ENOMEM: i32 = -12;
pub const ENOENT: i32 = -2;
pub const STALE_REGISTRY: i32 = -1101;
pub const POLICY: i32 = -1102;
pub const NOT_ENROLLED: i32 = -1103;
pub const PROVER: i32 = -1104;
pub const OUTSIDE: i32 = -1105;

/// `n` while it runs: fetched and wiped, never shown.
pub const STOPPED: &[u8] = b"Stopped. The proof is thrown away; this run cannot be proven again";
/// After Reading the run is used up: MkSealRead drops what it hands out.
pub const AGAIN: &[u8] = b"Run it again to prove it again";

/// The line under a failure: AGAIN once the run was read, else none.
pub fn again(past_reading: bool) -> Option<&'static [u8]> {
    past_reading.then_some(AGAIN)
}

/// The sentence for `fail`; `known` is whether the store saw the release
/// installed, so proved against its id rather than the default.
pub fn said(fail: Fail, name: &[u8], known: bool) -> Vec<u8> {
    let fixed: &[u8] = match fail {
        Fail::NoBox => b"The market has no box for this listing to prove against",
        Fail::NotNamed => b"The market's answer does not name this listing; not asked",
        Fail::MayNot => b"This window may not ask for a proof",
        Fail::Busy => b"The prover is busy: close its window, or try again in a few minutes",
        Fail::NotStarted => b"The prover did not start; the serial log says why",
        Fail::Quiet => b"The prover stopped answering",
        Fail::Broken => b"The proof came back broken",
        Fail::Code(ENOENT) => {
            let tail = b", close it, then press p";
            return [&b"No finished run to prove: run "[..], name, tail].concat();
        }
        Fail::Code(POLICY) if !known => {
            let tail = b" changed since it was installed; reinstall it to prove it";
            return [&b"The market's box for "[..], name, tail].concat();
        }
        Fail::Code(STALE_REGISTRY) => b"This machine's registry copy is out of date",
        Fail::Code(POLICY) => b"This run is not of this listing's box",
        Fail::Code(NOT_ENROLLED) => b"This device is not enrolled",
        Fail::Code(PROVER) => b"The proof did not come out right",
        Fail::Code(OUTSIDE) => b"A call outside its box succeeded: there is nothing true to prove",
        Fail::Code(ENOMEM) => b"Not enough memory for the prover",
        Fail::Code(_) => b"The prover stopped; the serial log says why",
    };
    fixed.to_vec()
}
