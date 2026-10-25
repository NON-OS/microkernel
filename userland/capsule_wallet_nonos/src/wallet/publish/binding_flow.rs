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
 * The publisher screen (plan/31, v1.4.16): read an entry document the
 * person named under /data, and sign its PublisherBinding once the person
 * has confirmed the key, the signature written beside the document.
 */

use alloc::string::String;

use nonos_marketplace_abi::MarketplaceEntry;
use nonos_nox_config::PUBLISHER_BINDING;

use super::super::publisher::{binding_plan, binding_sig_path, doc_path_ok, Refused};
use super::super::state::State;
use super::super::typed::ENTRY_BINDING;
use super::files::{read_doc, write_sig};
use super::kec::Kec;
use super::sign::sign_typed;

pub fn why(r: Refused) -> &'static str {
    match r {
        Refused::NotSold => "this listing sells nothing, so there is nothing to bind",
        Refused::NotPayee => "this listing pays another account, not this one",
        Refused::NotOurBond => "this listing names a bond that is not this account's for its key",
        Refused::Shape => "this release is neither an escrow nor a package release",
        Refused::NoRelease => "there is no such release",
    }
}

pub fn load(s: &State, path: &str) -> Result<MarketplaceEntry, &'static str> {
    if !doc_path_ok(path) {
        return Err("type a path under /data, with no .. and no spaces in it");
    }
    read_doc(s.vault.owner_pid, path)
}

pub fn sign_binding(
    s: &State,
    path: &str,
    e: &MarketplaceEntry,
    chain: u64,
) -> Result<String, &'static str> {
    let plan = binding_plan(&Kec, e, &s.acct.address).map_err(why)?;
    let at = binding_sig_path(path).ok_or("that path cannot hold a signature")?;
    let sig = sign_typed(s, ENTRY_BINDING, &PUBLISHER_BINDING, &plan.words, chain)?;
    write_sig(s.vault.owner_pid, &at, &sig).map(|_| at)
}
