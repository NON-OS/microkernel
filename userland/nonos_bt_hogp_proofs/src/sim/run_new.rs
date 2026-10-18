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

//! A run from a database, or from a peripheral told to misbehave.

use super::db::Db;
use super::run::Run;
use super::server::Peripheral;
use nonos_bt_hogp::host::Host;
use std::collections::VecDeque;

impl Run {
    pub fn new(db: Db) -> Self {
        Self::with(Peripheral::new(db))
    }

    pub fn with(peer: Peripheral) -> Self {
        let (host, first) = Host::start(0);
        let mut run = Self {
            host,
            peer,
            now: 0,
            sent: Vec::new(),
            input: Vec::new(),
            outcomes: Vec::new(),
            batteries: Vec::new(),
            to_peer: VecDeque::new(),
        };
        run.take(first);
        run
    }
}
