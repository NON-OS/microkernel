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

/*
 * One release's ReleaseAttest (plan/31, v1.4.18), through the log, which
 * keeps a second, different attest of a release from ever being signed. A
 * boot that keeps nothing signs none, since its log would forget.
 */

use alloc::string::String;

use nonos_marketplace_abi::MarketplaceEntry;
use nonos_nox_config::RELEASE_ATTEST;

use super::super::attest::{decide, signed, Decision, Refusal};
use super::super::publisher::{attest_of, attest_sig_path, Refused};
use super::super::state::State;
use super::super::typed::{attest_words, ENTRY_ATTEST};
use super::super::vault::persistent;
use super::binding_flow::why;
use super::files::{read_log, write_log, write_sig};
use super::kec::Kec;
use super::sign::sign_typed;

pub fn sign_attest(
    s: &State,
    path: &str,
    e: &MarketplaceEntry,
    release: usize,
    chain: u64,
) -> Result<String, &'static str> {
    if !persistent() {
        return Err(
            "this boot keeps nothing, so no attest is signed: the log could not remember it",
        );
    }
    let a = attest_of(&Kec, e, release, chain).map_err(why)?;
    let words = attest_words(&Kec, &s.address, &e.publisher_pubkey, &a)
        .map_err(|_| why(Refused::NotOurBond))?;
    let at = attest_sig_path(path, &e.releases[release].release_id)
        .ok_or("that release id cannot name a file")?;
    let log = read_log(s.vault.owner_pid)?;
    let sig = match decide(&log, &a) {
        Ok(Decision::Again(sig)) => sig,
        Ok(Decision::Sign) => {
            let sig = sign_typed(s, ENTRY_ATTEST, &RELEASE_ATTEST, &words, chain)?;
            write_log(s.vault.owner_pid, &signed(&log, &a, sig).map_err(|_| "the attest log is full")?)?;
            sig
        }
        Err(Refusal::Conflict) => return Err("a different attest of this release was signed before; a second would let the bond be slashed"),
        Err(Refusal::Full) => return Err("the attest log is full"),
    };
    write_sig(s.vault.owner_pid, &at, &sig).map(|_| at)
}
