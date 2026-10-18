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

//! The switches advanced security starts from.

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub enable_cet: bool,
    pub enable_smep: bool,
    pub enable_smap: bool,
    pub enable_umip: bool,
    pub enable_mpx: bool,
    pub enable_cfi: bool,
    pub enable_stack_canaries: bool,
    pub enable_aslr: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_cet: true,
            enable_smep: true,
            enable_smap: true,
            enable_umip: true,
            enable_mpx: true,
            enable_cfi: true,
            enable_stack_canaries: true,
            enable_aslr: true,
        }
    }
}
