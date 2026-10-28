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

//! Setup's answers across boots: kept in the store of the disk this boot
//! came from only by a mode that keeps state there (`choice`), held in
//! memory for the installer otherwise, and read back so a later boot from a
//! disk that kept them skips setup.

mod choice;
mod put;
mod save;
mod skip;
mod store;

pub use choice::{answers_for, Answers, INSTALL, USB_LIVE};
pub use save::{save, stage};
pub use skip::{already_done, wait_for_policy};
pub use store::{store_pending, store_ready};
