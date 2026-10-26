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

use crate::elf::errors::ElfResult;
use crate::elf::loader::validate_elf as validate_header;

use super::parse::parse_header;

pub fn validate_elf(bytes: &[u8]) -> bool {
    validate_elf_detailed(bytes).is_ok()
}

pub fn validate_elf_detailed(bytes: &[u8]) -> ElfResult<()> {
    let header = parse_header(bytes)?;
    validate_header(&header)
}
