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

//! The loader's rollback counter, its own files: the commands, the read's
//! mapping and the read and raise sequences, here run against swtpm (REVIEW
//! R20). boot_proofs drives the same files against a scripted TPM.

#[path = "../../../../nonos-bootloader/src/security/tpm_nv/consts.rs"]
pub mod consts;
#[path = "../../../../nonos-bootloader/src/security/tpm_nv/floor_cmd.rs"]
pub mod floor_cmd;
#[path = "../../../../nonos-bootloader/src/security/tpm_nv/floor_seq.rs"]
pub mod floor_seq;
