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
 * One of the meter's channels as a line (B8): where it is, what it holds,
 * what the chain paid and what the meter signed, and whether it froze.
 */

use alloc::format;
use alloc::string::String;

use nonos_payment_proto::AccountRow;

fn chain(id: u64) -> String {
    match id {
        1 => String::from("mainnet"),
        11_155_111 => String::from("Sepolia"),
        other => format!("chain {other}"),
    }
}

/// One channel: where, what it holds, what the chain paid, what was signed.
pub fn channel_line(r: &AccountRow, nox: fn(u128) -> String) -> String {
    let frozen = if r.frozen { " · frozen: settled above what this meter signed" } else { "" };
    format!(
        "{} · {} held · {} paid · {} signed{frozen}",
        chain(r.chain_id),
        nox(r.deposit),
        nox(r.settled),
        nox(r.cumulative)
    )
}
