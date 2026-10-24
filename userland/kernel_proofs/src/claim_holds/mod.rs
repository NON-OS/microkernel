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

/*
 * Which devices the broker keeps from whichever capsule asks first: VMMDev
 * for the capsule that serves it, virtio-balloon for no one yet, the boot
 * framebuffer for driver.bootfb0 alone; the records firmware tables publish
 * for the service that drives each kind, the SCI's for the ACPI capsule,
 * an SD host's for the SD host capsule, any other refused (L59); the VMBus
 * root for the VMBus core alone; a device-tree xHCI controller for the xHCI
 * capsule. The kernel's pure rules are included by path.
 */

mod firmware_tests;
#[path = "../../../../src/hardware/broker/claim/hold_acpi.rs"]
pub mod hold_acpi;
#[path = "../../../../src/hardware/broker/claim/hold_platform.rs"]
pub mod hold_platform;
#[path = "../../../../src/hardware/broker/claim/hold_rule.rs"]
pub mod hold_rule;
#[path = "../../../../src/hardware/broker/claim/hold_tree.rs"]
pub mod hold_tree;
#[path = "../../../../src/hardware/broker/claim/hold_virt.rs"]
pub mod hold_virt;
mod platform_sci_tests;
mod platform_sd_tests;
mod platform_tests;
mod tests;
mod tree_tests;
mod virt_tests;
