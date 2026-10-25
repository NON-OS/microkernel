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

//! A grant on its own: one that parses obeys every grant rule, and the
//! bytes its issuer signed are a prefix of it.

#![no_main]

mod rules;

use libfuzzer_sys::fuzz_target;
use nsh_wire::Grant;

fuzz_target!(|data: &[u8]| {
    if let Ok(g) = Grant::parse(data) {
        assert!(data.starts_with(g.signed));
        rules::hold(&g);
    }
});
