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
 * Which hypervisor the kernel believes it runs under. The classifier and the
 * PCI refinement are included by path and fed the CPUID words each vendor
 * documents, so a wrong table entry fails here rather than in a boot log.
 */

#[path = "../../../../src/arch/x86_64/hypervisor/backdoor_rule.rs"]
mod backdoor_rule;
#[path = "../../../../src/arch/x86_64/hypervisor/balloon.rs"]
mod balloon;
#[path = "../../../../src/arch/x86_64/hypervisor/classify.rs"]
mod classify;
#[path = "../../../../src/arch/x86_64/hypervisor/guest.rs"]
mod guest;
#[path = "../../../../src/arch/x86_64/hypervisor/leaves.rs"]
mod leaves;
#[path = "../../../../src/arch/x86_64/hypervisor/refine.rs"]
mod refine;
mod tests;
mod tests_backdoor;
mod tests_balloon;
mod tests_codes;
mod tests_refine;
#[path = "../../../../src/arch/x86_64/hypervisor/vendor.rs"]
mod vendor;
