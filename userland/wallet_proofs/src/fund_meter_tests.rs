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

//! A withdrawal to the meter (B7): only from the config's Shield pool, only
//! to a meter whose key is open, and the wallet's NOX is the config's.

use nonos_nox_config::{address, Contract, MAINNET, SEPOLIA};
use nonos_payment_proto::{Account, KEPT, READY};

use crate::fund_meter::NO_METER;
use crate::fund_meter::{meter_destination, pool_is_configured, METER_NOT_READY, NOT_THIS_POOL};

const POOL: &str = "0xaEe51E82965Ec1DeD870F3f4c248Ad4AdDc3e1cb";

fn account(flags: u8, address: [u8; 20]) -> Account {
    Account { flags, address, cap: 0, spent: 0, expiry: 0, left: 0, total: 0, rows: vec![] }
}

#[test]
fn only_the_configs_pool_on_its_chain_funds_the_meter() {
    assert!(pool_is_configured(POOL, SEPOLIA));
    assert!(pool_is_configured(&POOL.to_ascii_lowercase(), SEPOLIA), "case does not matter");
    assert!(!pool_is_configured(POOL, MAINNET), "no pool on mainnet until O3");
    assert!(!pool_is_configured("0xaEe51E82965Ec1DeD870F3f4c248Ad4AdDc3e1cc", SEPOLIA));
    assert!(!pool_is_configured("0xaEe51E82", SEPOLIA));
    let ready = account(READY | KEPT, [7; 20]);
    assert_eq!(meter_destination(POOL, MAINNET, Some(&ready)), Err(NOT_THIS_POOL));
}

#[test]
fn the_meter_is_paid_only_with_its_key_open() {
    let ready = account(READY, [7; 20]);
    assert_eq!(meter_destination(POOL, SEPOLIA, Some(&ready)), Ok([7; 20]));
    assert_eq!(
        meter_destination(POOL, SEPOLIA, Some(&account(KEPT, [7; 20]))),
        Err(METER_NOT_READY)
    );
    assert_eq!(
        meter_destination(POOL, SEPOLIA, Some(&account(READY, [0; 20]))),
        Err(METER_NOT_READY)
    );
    assert_eq!(meter_destination(POOL, SEPOLIA, None), Err(NO_METER));
}

#[test]
fn the_wallets_nox_is_the_configs() {
    use crate::wallet::chain;
    assert_eq!(Ok(chain::MAINNET.nox), address(MAINNET, Contract::NoxToken));
    assert_eq!(Ok(chain::SEPOLIA.nox), address(SEPOLIA, Contract::NoxToken));
}
