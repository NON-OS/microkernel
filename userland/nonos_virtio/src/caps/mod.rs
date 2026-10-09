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

//! The virtio vendor capabilities: where the register structures live.

mod layout;
mod msix;
mod parse;
mod region;
mod types;
mod validate;
mod walk;

pub use layout::{
    CAP_FIRST, CFG_COMMON, CFG_DEVICE, CFG_ISR, CFG_NOTIFY, CFG_PCI, COMMON_CFG_LEN, MAX_CAPS,
};
pub use msix::MsixLayout;
pub use parse::parse;
pub use region::{Region, PAGE_SIZE};
pub use types::ModernCaps;
pub use walk::CapWalk;
