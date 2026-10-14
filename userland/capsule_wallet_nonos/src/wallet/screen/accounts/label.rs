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

//! The account rows' words: each account's number and short address,
//! the open one marked.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::wallet::etna::groups::shortened;
use crate::wallet::state::State;

fn hex(address: &[u8; 20]) -> String {
    let mut out = String::from("0x");
    for b in address {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

pub(super) fn labels(state: &State) -> Vec<String> {
    state
        .acct
        .list
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let open = if i == state.acct.open as usize { "  open" } else { "" };
            format!("Account {}  {}{open}", a.index, shortened(&hex(&a.address)))
        })
        .collect()
}
