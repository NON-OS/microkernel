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

//! Leaving UEFI on aarch64: the shared hand-off steps, then the arm64 boot
//! protocol (arch::aarch64::exit). No page tables are built here: the
//! kernel enters with its MMU off and builds its own.

use uefi::prelude::*;

use super::cleanup::secure_cleanup_before_jump;
use super::final_map::final_map;
use super::gather::gather_system_info;
use super::handoff_init::init_boothandoff;
use super::no_tree::refuse_no_tree;
use crate::arch::aarch64::exit::{device_tree, enter_kernel};
use crate::firmware::FirmwareHandoff;
use crate::handoff::jump::settle_delay;
use crate::handoff::prepare::allocate_handoff_resources;
use crate::handoff::types::{BootHandoffV1, CryptoHandoff, InstallHandoff};
use crate::loader::KernelImage;

#[allow(clippy::too_many_arguments)]
pub fn exit_and_jump(
    st: SystemTable<Boot>,
    kernel: &KernelImage,
    cmdline: Option<&str>,
    crypto: CryptoHandoff,
    firmware: FirmwareHandoff,
    rng_seed: [u8; 32],
    tpm_measured: bool,
    install: InstallHandoff,
) -> ! {
    let Some(dtb) = device_tree(&st) else { refuse_no_tree(st) };
    let bs = st.boot_services();
    let allocs = allocate_handoff_resources(&st, cmdline);
    let params = gather_system_info(
        &st,
        bs,
        kernel,
        &crypto,
        firmware,
        tpm_measured,
        &allocs,
        rng_seed,
        install,
    );
    let bh_ptr = allocs.boothandoff_addr as *mut BootHandoffV1;
    // SAFETY: `allocate_handoff_resources` gave a page of loader data here.
    unsafe { init_boothandoff(bh_ptr, &params) };

    crate::display::gop::shutdown_for_exit();
    secure_cleanup_before_jump();
    settle_delay();
    let (_runtime_st, final_mmap) = st.exit_boot_services();
    let map = final_map(&allocs, bh_ptr, &final_mmap);
    enter_kernel(kernel.entry_point as u64, dtb, allocs.boothandoff_addr, map)
}
