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

use super::super::config::ConfigSpace;
use super::super::error::Result;

pub fn disable_legacy_interrupt(config: &ConfigSpace) -> Result<()> {
    config.disable_interrupts()
}

pub fn enable_legacy_interrupt(config: &ConfigSpace) -> Result<()> {
    config.enable_interrupts()
}

pub fn get_interrupt_line(config: &ConfigSpace) -> Result<u8> {
    config.interrupt_line()
}

pub fn get_interrupt_pin(config: &ConfigSpace) -> Result<u8> {
    config.interrupt_pin()
}
