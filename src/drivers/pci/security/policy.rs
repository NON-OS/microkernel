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

use spin::Mutex;

/// The config-write rules validation.rs applies, fixed at build time. The
/// setter and getter nothing called are gone: a policy no caller can
/// change is one less thing in the TCB.
pub struct SecurityPolicy {
    pub allow_rom_writes: bool,
    pub allow_arbitrary_bus_master: bool,
    pub allow_interrupt_line_writes: bool,
    pub log_all_config_writes: bool,
    pub enforce_allowlist: bool,
    pub block_unknown_vendors: bool,
}

pub(super) static POLICY: Mutex<SecurityPolicy> = Mutex::new(SecurityPolicy {
    allow_rom_writes: false,
    allow_arbitrary_bus_master: false,
    allow_interrupt_line_writes: true,
    log_all_config_writes: true,
    enforce_allowlist: false,
    block_unknown_vendors: false,
});
