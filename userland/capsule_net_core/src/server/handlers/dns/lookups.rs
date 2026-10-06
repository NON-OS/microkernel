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

//! The lookups whose callers wait for an answer, and how long they may.

use smoltcp::socket::dns::QueryHandle;
use spin::Mutex;

use super::pending::Table;
use crate::state::{DnsSockets, DNS_SERVERS};

/// How long a lookup may wait for its answer.
pub(super) const TIMEOUT_MS: i64 = 3000;

/// Where a waiting lookup's queries live: the socket set, the DNS socket of
/// each server the lease named, and the query on each. A query is dropped
/// here once its result is read: smoltcp frees its slot then and may give
/// the slot to another lookup.
#[derive(Clone, Copy)]
pub struct Lookup {
    pub(super) generation: u32,
    pub(super) dns: DnsSockets,
    pub(super) queries: [Option<QueryHandle>; DNS_SERVERS],
}

pub(super) static PENDING: Mutex<Table<Lookup>> = Mutex::new(Table::new());

/// Whether a lookup is waiting, so the loop polls the device for its answer.
pub fn waiting() -> bool {
    PENDING.lock().any()
}
