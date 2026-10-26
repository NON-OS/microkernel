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

//! An Arm firmware that gives no device tree. The kernel takes its whole
//! platform from the tree, so the boot stops here, and says why on the
//! console. It halts rather than resets: a reset would only come back here.

use uefi::prelude::*;

use crate::log::logger::log_error;

pub fn refuse_no_tree(mut st: SystemTable<Boot>) -> ! {
    log_error("handoff", "aarch64: the firmware gives no device tree");
    let out = st.stdout();
    let _ = out.output_string(cstr16!(
        "\r\n[FATAL] This firmware gives ACPI tables and no device tree.\r\n"
    ));
    let _ = out
        .output_string(cstr16!("NONOS on Arm reads its platform from a device tree. On QEMU,\r\n"));
    let _ = out.output_string(cstr16!("start the virt board with acpi=off. Halted.\r\n"));
    crate::arch::cpu::halt_forever()
}
