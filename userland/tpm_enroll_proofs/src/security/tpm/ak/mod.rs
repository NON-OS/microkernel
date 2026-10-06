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

//! The attestation key's files, as the kernel has them.

#[path = "../../../../../../src/security/tpm/ak/attributes.rs"]
mod attributes;
mod command;
#[path = "../../../../../../src/security/tpm/ak/create.rs"]
mod create;
#[path = "../../../../../../src/security/tpm/ak/cursor.rs"]
pub mod cursor;
#[path = "../../../../../../src/security/tpm/ak/identity.rs"]
mod identity;
#[path = "../../../../../../src/security/tpm/ak/load.rs"]
mod load;
#[path = "../../../../../../src/security/tpm/ak/public.rs"]
mod public;
#[path = "../../../../../../src/security/tpm/ak/template.rs"]
pub mod template;

pub use command::create_command;
pub use identity::ak_public;
pub use load::{ak_handle, load_ak};
