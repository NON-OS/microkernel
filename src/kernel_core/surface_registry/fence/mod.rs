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

//! A surface's release fence (MkSurfaceFenceRelease, MkSurfaceFenceWait).
//! A count per slot that a holder of the surface moves on when it is done
//! reading it, and that the project waits on, so a swapchain client draws a
//! buffer again only once the compositor has let it go. Keyed by the slot,
//! not by an address, so neither side can name the other's memory.

mod count;
mod release;
mod wait;
pub(crate) mod wait_rule;
pub mod waiters;
mod waiters_list;

pub use release::release_fence;
pub use wait::wait_fence;
pub use waiters::forget as forget_fence_waiter;

pub(super) use release::wake_slot;
