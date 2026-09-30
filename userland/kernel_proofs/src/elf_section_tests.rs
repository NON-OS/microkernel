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
 * is_symtab must name the full symbol table only.
 *
 * It accepted SHT_DYNSYM as well, so get_symbol_table returned .dynsym on an
 * image that places it first. The check below fails against that code.
 */

use crate::elf::loader::core::section::ParsedSection;

fn section(section_type: u32) -> ParsedSection {
    ParsedSection {
        name: "s".into(),
        section_type,
        flags: 0,
        addr: 0,
        offset: 0,
        size: 0,
        link: 0,
        info: 0,
        alignment: 0,
        entry_size: 0,
    }
}

#[test]
fn only_sht_symtab_is_a_symbol_table() {
    assert!(section(2).is_symtab());
    assert!(!section(11).is_symtab());
    for t in (0..64).filter(|&t| t != 2) {
        assert!(!section(t).is_symtab(), "type {t}");
    }
}
