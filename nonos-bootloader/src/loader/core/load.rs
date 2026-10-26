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

use uefi::prelude::*;

use super::dynamic::load_dyn_kernel;
use super::elf_validate::validate_elf;
use super::exec::image_rules::{route, Route};
use super::exec::load_exec_kernel;
use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::image::KernelImage;
use crate::loader::types::memory;
use crate::log::logger::{log_error, log_info};

pub fn load_kernel(
    system_table: &mut SystemTable<Boot>,
    payload: &[u8],
) -> LoaderResult<KernelImage> {
    log_info("loader", "Starting kernel load operation");

    let validation = validate_elf(payload)?;
    let bs = system_table.boot_services();

    // A self-relocating ET_DYN (aarch64) is placed like an ET_EXEC and
    // left for its own relocator, which writes every slot afresh.
    let upper_half = memory::is_upper_half(validation.min_addr);
    let relocating = crate::arch::cpu::IMAGE_ALIGN.is_some();
    match route(validation.is_exec, validation.is_dyn, upper_half, relocating) {
        Route::Exec | Route::Block => load_exec_kernel(bs, payload, &validation),
        Route::Dyn => load_dyn_kernel(bs, payload, &validation),
        Route::Refuse(why) => {
            log_error("loader", why);
            Err(LoaderError::UnsupportedElf(why))
        }
    }
}
