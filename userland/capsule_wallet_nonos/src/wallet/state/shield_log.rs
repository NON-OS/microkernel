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
 * The shield's history as the service reads it from the account's own
 * store, so it is the same after a restart. Nothing in the wallet invents
 * an entry or moves one forward on its own.
 */

use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    /* Sent or proved, and not seen on chain yet. */
    Sent,
    /* Its owner sent the settlement, not seen on chain yet. */
    Settling,
    /* In the pool: a deposit stored, a spend landed, a payment received. */
    Settled,
    /* Taken back: it never landed and its notes are spendable again. */
    Failed,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Deposit,
    Approval,
    Send,
    Withdraw,
    SelfSettle,
    /* Payments to the nox1 address the pool was read to hold. */
    Received,
}

pub struct Entry {
    pub kind: Kind,
    pub asset: u8,
    pub amount: String,
    pub stage: Stage,
    pub tx: Option<String>,
}
