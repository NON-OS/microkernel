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

mod keep_tests;
mod move_tests;
mod pci_tests;
mod refusal_tests;
mod reveal_tests;
mod rig;
mod sleep_facts_tests;
mod wake_depart_tests;
mod wake_event_tests;
mod wake_gone_tests;
mod wake_more_tests;
mod wake_present_tests;
mod wake_rig;
mod wake_tests;
// The capsule's own sweep, by path, proven against the real disarm;
// its read check is the capsule's proofs'.
#[path = "../../../../capsule_acpi/src/dpower/wake_sweep.rs"]
mod wake_sweep;
