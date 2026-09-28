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
 * What the shield service reports back: each shielded action with where it
 * has got to, and the proof being built now. Written by the service, read by
 * the screens; nothing in the wallet invents an entry.
 */

use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Proving,
    HandedOff,
    Settled,
    Failed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Deposit,
    Send,
    Receive,
    Withdraw,
}

pub struct Entry {
    pub kind: Kind,
    pub asset: u8,
    pub amount: String,
    pub stage: Stage,
    pub tx: Option<[u8; 32]>,
}

pub struct Job {
    pub kind: Kind,
    pub elapsed_s: u32,
    pub done: u16,
    pub of: u16,
    pub error: Option<&'static str>,
}
