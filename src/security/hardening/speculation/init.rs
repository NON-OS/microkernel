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

/// Detect what this CPU is vulnerable to and turn on what it supports.
pub fn init() -> Result<(), &'static str> {
    #[cfg(target_arch = "x86_64")]
    return super::super::spectre_mitigations::init();

    // Each aarch64 CPU decides and acts for itself in
    // `arch::aarch64::security::init_all`, before it can enter EL0. Running
    // it again here would act for whichever CPU this happens to be on.
    #[cfg(target_arch = "aarch64")]
    return Ok(());

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    Ok(())
}
