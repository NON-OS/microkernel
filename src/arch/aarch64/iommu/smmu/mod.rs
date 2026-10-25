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

//! The ARM SMMUv3: confining device DMA on aarch64, the counterpart of
//! VT-d. Stage 1 translation, a two-level stream table whose every stream
//! aborts until a capsule's domain claims it, and bounded waits throughout.

pub mod backend;
mod cmdq;
mod cmdq_sync;
mod domain;
pub mod encode;
mod eventq;
mod frame;
mod gerror;
mod init;
mod irq;
mod mmio;
mod pgtable;
mod pgtable_unmap;
mod refusal;
mod regs;
mod reset;
mod state;
mod ste_write;
mod strtab;
mod strtab_write;
mod wait;
mod walk;
mod window;

pub use init::{bring_up, describe};
pub use state::in_service_phandle;
pub use window::described_window;
