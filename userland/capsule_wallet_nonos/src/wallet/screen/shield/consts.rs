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
 * The Sepolia deployment the Shield screens name, and the privacy rules they
 * state. Copied from the contracts lane's deployment record; the pool is the
 * contract that refuses any amount off the standard ladder.
 */

pub const CHAIN_ID: &str = "11155111";
pub const POOL: &str = "0xD0dBCe195c082DA39a218C62c01a732CE5b4d541";
pub const POOL_FROM_BLOCK: &str = "11,786,912";
pub const VERIFIER: &str = "0xde6110142b39730f480f9d66c408f150a7e823c1";
pub const FAUCET: &str = "0x871bc3AD5DA20c399d631817637cB5FB29eB04B4";
pub const RELAYER: &str = "j3fsu7dad6cmgi3r2i63dsgkkltrhg64z6hf6xajffajjwj4elvke6qd.onion";

/* The asset numbers the note store is keyed by. The scanner that fills the
 * store must record notes under the same two. */
pub const NOTE_ETH: u32 = 0;
pub const NOTE_NOX: u32 = 1;

/* One flat relay fee, in the asset being moved. */
pub const FEE_ETH: &str = "0.00005 ETH";
pub const FEE_NOX: &str = "5 NOX";

/* A note is spendable once the pool has grown by this much, and this long. */
pub const WAIT_LEAVES: u32 = 20;
pub const WAIT_HOURS: u32 = 6;
/* What the holder types to spend a note before the wait is over. */
pub const OVERRIDE: &str = "spend early";

pub const SEND_LEAD: &str = "The proof hides who paid whom and how much. \
     Deposits and withdrawals are public standard amounts.";

pub fn fee(asset: u8) -> &'static str {
    if asset == crate::wallet::state::shield_ui::ASSET_NOX {
        FEE_NOX
    } else {
        FEE_ETH
    }
}

pub fn ticker(asset: u8) -> &'static str {
    if asset == crate::wallet::state::shield_ui::ASSET_NOX {
        "NOX"
    } else {
        "ETH"
    }
}
