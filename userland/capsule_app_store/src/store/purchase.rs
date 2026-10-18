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

//! 19/0 POST (plan/21, plan/27): the selected listing's names, and only its
//! names, into the meter's mailbox for the wallet to take.

use alloc::vec::Vec;

use nonos_nox_config::SEPOLIA;
use nonos_payment_proto::{Names, EINVAL, OP_PURCHASE};

use super::asked::Asked;
use super::listing::Listing;
use super::meter_call::call;
use super::purchase_said::handed;
use super::state::State;

/// The names a listing's purchase goes by: the listing's default release.
pub fn names_of(l: &Listing) -> Names {
    Names { chain_id: SEPOLIA, listing: l.id.clone(), release: Vec::new() }
}

pub fn post(state: &State) -> Asked {
    let Some(l) = state.current() else { return Asked::NotInstallable };
    let body = names_of(l).post().ok_or(EINVAL);
    handed(body.and_then(|b| call(OP_PURCHASE, &b, 0)).map(|_| ()))
}
