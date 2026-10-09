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

//! nonos.shield: the NOX Shield wallet the phone apps run, as a service the
//! NONOS wallet calls.
//!
//! It holds one `Wallet` of the phones' core under `/data/shield`, sealed to
//! this machine, and answers the wallet window over IPC: open it from the
//! recovery words, sync it with the pool, review and send deposits, prove
//! and publish private payments and withdrawals, follow them, and settle one
//! from the public account when nobody else will. Every byte it sends goes
//! over the anonymity network the machine has chosen; it never goes direct.

mod guard;
mod jobs;
mod ops;
mod periodic;
mod pool;
mod random;
mod reply;
mod service;
mod steps;

fn main() {
    pool::start();
    service::run();
}
