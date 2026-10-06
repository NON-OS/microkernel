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

//! Where the kernel's device secret reads the loader's approval file. No loader
//! runs under a host test, so there is no handoff and `from_boot` reads none;
//! the tests hand `device_secret` their own approvals.

pub struct Policy {
    pub approval_present: u8,
    pub approval: [u8; 128],
}

pub struct Handoff {
    pub policy: Policy,
}

pub fn get_handoff() -> Option<&'static Handoff> {
    None
}
