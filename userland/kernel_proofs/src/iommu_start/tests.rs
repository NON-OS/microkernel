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

//! no enumerated function starts identity mapped. One with
//! no reserved memory starts blocked; one with reserved memory gets a domain
//! of only that memory, or stays blocked if one cannot be made.

use super::start::{start_for, Start};

#[test]
fn every_device_starts_blocked_or_in_its_reserved_memory_only() {
    assert_eq!(start_for(false, false), Start::Blocked, "a NIC, a disk, a Thunderbolt port");
    assert_eq!(start_for(false, true), Start::Blocked, "no reserved memory, no domain of its own");
    assert_eq!(start_for(true, true), Start::Reserved, "graphics stolen memory, USB legacy");
    assert_eq!(start_for(true, false), Start::Blocked, "no domain left: blocked, not identity");
}
