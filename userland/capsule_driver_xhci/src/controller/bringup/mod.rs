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
//! Taking the controller from firmware, halting and resetting it, giving it
//! its DMA structures, and starting it.
mod alloc_dcbaa;
mod halt;
mod layout;
mod legacy_handoff;
mod program_command_ring;
mod program_dcbaa;
mod program_event_ring;
mod refuse_unsupported;
mod reset;
pub(crate) mod scratchpad;
mod start;
mod wait_cnr_clear;
mod wait_hc_running;
pub use alloc_dcbaa::alloc_dcbaa;
pub use halt::halt;
pub use layout::ControllerLayout;
pub use legacy_handoff::legacy_handoff;
pub use program_command_ring::program_command_ring;
pub use program_dcbaa::program_dcbaa;
pub use program_event_ring::program_event_ring;
pub use refuse_unsupported::refuse_unsupported;
pub use reset::reset;
pub use scratchpad::Scratchpads;
pub use start::start;
pub use wait_cnr_clear::wait_cnr_clear;
pub use wait_hc_running::wait_hc_running;
