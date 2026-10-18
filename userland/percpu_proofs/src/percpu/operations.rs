// NONOS Operating System (AGPL-3.0-or-later)
//! Stand-in for the kernel's block array: four blocks, in one array, as the
//! kernel keeps them, so `by_base` is exercised over real addresses.

use super::types::PerCpuData;

pub const BLOCKS: usize = 4;

pub static ALL: [PerCpuData; BLOCKS] = [const { PerCpuData::new() }; BLOCKS];

pub fn get(id: usize) -> Option<&'static PerCpuData> {
    ALL.get(id)
}
