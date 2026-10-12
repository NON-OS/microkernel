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

//! The report descriptor walk and the reads made through what it finds,
//! over the descriptors real devices send and hostile ones.

mod bind_tests;
mod binds;
mod boot_fallback_tests;
mod bound_tests;
mod build;
mod decode_tests;
mod device_tests;
mod fuzz_reads;
mod fuzz_tests;
mod hostile_gen;
mod hostile_tests;
mod id_tests;
mod layout_more_tests;
mod layout_of;
mod layout_tests;
mod media_tests;
mod nesting_tests;
mod qemu;
mod qemu_tablet_range_tests;
mod qemu_tablet_tests;
mod read_tests;
mod tablet_stream_tests;
pub mod vectors;
mod vectors_more;

pub use layout_of::layout;
