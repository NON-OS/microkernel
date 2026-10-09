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

//! The platform's switches, as the security checks found them.

use super::rows::Row;
use super::state::State;
use crate::display::text::Text;
use crate::security::SecurityContext;

pub fn secure_boot(s: &SecurityContext) -> Row {
    switch(
        b"SECURE BOOT",
        s.secure_boot_enabled,
        b"UEFI Secure Boot enabled",
        b"UEFI Secure Boot disabled",
    )
}

pub fn tpm(s: &SecurityContext) -> Row {
    switch(
        b"TPM",
        s.measured_boot_active,
        b"TPM 2.0 measured boot active",
        b"no TPM 2.0 measured boot",
    )
}

fn switch(label: &'static [u8], on: bool, yes: &[u8], no: &[u8]) -> Row {
    let (state, detail) = if on { (State::On, yes) } else { (State::Off, no) };
    Row { label, state, detail: Text::new().push(detail) }
}
