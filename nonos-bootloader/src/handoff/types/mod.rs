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

mod boot_media;
mod constants;
mod crypto;
mod framebuffer;
mod handoff;
mod install;
mod memory;
mod security;
mod system;

pub use boot_media::{BootMedia, BOOT_MEDIA_LEN};
pub use constants::{flags, HANDOFF_MAGIC, HANDOFF_VERSION};
pub use crypto::CryptoHandoff;
pub use framebuffer::FramebufferInfo;
pub use handoff::BootHandoffV1;
pub use install::InstallHandoff;
pub use memory::MemoryMap;
pub use security::{Measurements, RngSeed, ZkAttestation};
pub use system::{
    AcpiInfo, Module, Modules, SmbiosInfo, Timing, MODULE_KIND_BOOT_MEDIA,
    MODULE_KIND_KERNEL_IMAGE, MODULE_KIND_LOADER_IMAGE,
};
