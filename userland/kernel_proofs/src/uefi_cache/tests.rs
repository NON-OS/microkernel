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

use super::table::manager_with_firmware;
use crate::arch::x86_64::uefi::types::Guid;

#[test]
fn the_security_cache_keeps_the_firmware_attribute_word() {
    let (manager, _table) = manager_with_firmware();
    manager.cache_security_variables();
    let cache = manager.variables_cache.read();
    let pk = cache.get(&(String::from("PK"), Guid::GLOBAL_VARIABLE)).expect("PK cached");
    assert_eq!(pk.attributes.bits(), 0x27);
    assert!(pk.attributes.requires_authentication());
    let sb =
        cache.get(&(String::from("SecureBoot"), Guid::GLOBAL_VARIABLE)).expect("SecureBoot cached");
    assert_eq!(sb.attributes.bits(), 0x06);
    assert!(!sb.attributes.is_non_volatile());
}

#[test]
fn a_cache_miss_keeps_the_firmware_attribute_word() {
    let (manager, _table) = manager_with_firmware();
    let var = manager.get_variable("dbx", &Guid::IMAGE_SECURITY_DATABASE).expect("read");
    assert_eq!(var.attributes.bits(), 0x27);
    let again = manager.get_variable("dbx", &Guid::IMAGE_SECURITY_DATABASE).expect("cached");
    assert_eq!(again.attributes.bits(), 0x27);
}
