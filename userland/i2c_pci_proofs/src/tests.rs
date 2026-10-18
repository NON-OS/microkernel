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

//! The proofs, one file each beside lib.rs; the window model and the
//! bring-up setup they share stay at the crate root.

#[path = "clock_tests.rs"]
mod clock_tests;
#[path = "dma_tests.rs"]
mod dma_tests;
#[path = "gpio0_tests.rs"]
mod gpio0_tests;
#[path = "enable_state_tests.rs"]
mod enable_state_tests;
#[path = "lpss_tests.rs"]
mod lpss_tests;
#[path = "mmio_tests.rs"]
mod mmio_tests;
#[path = "narrow_tests.rs"]
mod narrow_tests;
#[path = "reach_tests.rs"]
mod reach_tests;
#[path = "regs_tests.rs"]
mod regs_tests;
#[path = "request_refusal_tests.rs"]
mod request_refusal_tests;
#[path = "reset_tests.rs"]
mod reset_tests;
#[path = "scl_count_tests.rs"]
mod scl_count_tests;
#[path = "scl_tests.rs"]
mod scl_tests;
#[path = "speed_tests.rs"]
mod speed_tests;
#[path = "table_tests.rs"]
mod table_tests;
#[path = "target_tests.rs"]
mod target_tests;
#[path = "unlisted_tests.rs"]
mod unlisted_tests;
