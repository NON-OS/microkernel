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

pub mod apic;
pub mod broker;
pub mod entry_flags;
pub mod ioapic;
pub mod irq_flag;
pub mod pic;

pub use apic::{ApicError, ApicResult, ApicStatus};
pub use entry_flags::clear_ac;
pub use ioapic::{IoApicError, IoApicResult, IoApicStatus};
pub use irq_flag::{cli, sti};
pub use pic::{PicError, PicResult, PicStatus};

pub use ioapic as nonos_ioapic;
