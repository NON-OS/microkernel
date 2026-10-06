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

//! The enrollment commands against a real TPM 2.0 implementation, with
//! tpm2-tools as the registrar's reference.

mod activate_tests;
mod cert_ecc_tests;
mod cert_setup;
mod cert_tests;
mod device_secret_tests;
mod dir;
mod ek_tests;
mod floor_tests;
mod quote_tests;
mod release;
mod resend_tests;
mod sign_tests;
pub(crate) mod steps;
pub(crate) mod swtpm;
mod swtpm_io;
pub(crate) mod tools;
pub(crate) mod verify;
