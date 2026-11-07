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

//! The good run brought up to AUTH_OK.

use super::roles::Table;
use super::roles_build::grant::GrantB;
use super::run::Run;

impl Run {
    /// The good run up to `AUTH_OK`: the shipped table, the default grant.
    pub fn logged_in() -> Run {
        let mut run = Run::new(&GrantB::default(), Table::shipped());
        run.pump();
        assert!(run.c_events.iter().any(|e| matches!(e, nsh_client::Event::LoggedIn(_))));
        run
    }
}
