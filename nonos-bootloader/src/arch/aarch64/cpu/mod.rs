// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The aarch64 answers to `arch::cpu`, from the Arm Architecture Reference
//! Manual's generic timer, ID registers and FEAT_RNG.

mod counter;
mod halt;
mod id_fields;
mod image_header;
mod platform;
mod ports;
mod rng;
mod security;

pub use counter::{cycles, cycles_serialized};
pub use halt::{halt_forever, reset};
pub use id_fields::IdRegs;
pub use image_header::image_size;
pub use platform::{ELF_MACHINE, IMAGE_ALIGN, PC_TPM_WINDOW, REQUIRE_STRONG_ENTROPY};
pub use ports::{halt_with_code, pci_config_read, post_byte};
pub use rng::{hw_random, hw_seed};
pub use security::{id_regs, security_features};
