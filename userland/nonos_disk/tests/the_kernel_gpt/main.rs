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

//! The kernel's own GPT code, mounted by path once for every claim made
//! about it (one file each below), with the disks and tables they build.

extern crate alloc;

#[path = "../common/entropy.rs"]
mod entropy;
#[path = "../common/files.rs"]
mod files;
#[path = "../common/fixed_sweep.rs"]
mod fixed_sweep;
#[path = "../common/fixed_tables.rs"]
mod fixed_tables;
#[path = "../common/gpt_damage.rs"]
mod gpt_damage;
#[path = "../common/gpt_sweep/mod.rs"]
mod gpt_sweep;
#[path = "../common/kernel_gpt/mod.rs"]
mod kernel_gpt;
#[path = "../common/mem_disk.rs"]
mod mem_disk;
#[path = "../common/written.rs"]
mod written;

mod the_kernel_finds_its_partitions_in_the_table;
mod the_kernel_gpt_holds_under_seeded_lies;
mod the_kernel_keeps_fixed_sectors_only_clear_of_foreign_partitions;
mod the_kernel_places_nothing_without_one_of_each;
mod the_kernel_reads_gpt_entries_larger_than_128_bytes;
mod the_kernel_refuses_a_gpt_header_firmware_would_not;
mod the_kernel_refuses_entries_it_cannot_place;
mod the_loaders_record_names_its_disk_to_the_kernel;
mod the_store_window_lands_in_the_store_partition;
