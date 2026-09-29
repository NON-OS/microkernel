// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/port/types/range.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/port/types/range.rs"]
pub mod range;

pub fn portrange_new(start: u16, count: u16) -> range::PortRange {
    range::PortRange::new(start, count)
}

pub fn portrange_start(this: range::PortRange) -> u16 {
    this.start()
}

pub fn portrange_count(this: range::PortRange) -> u16 {
    this.count()
}

pub fn portrange_end(this: range::PortRange) -> u16 {
    this.end()
}

pub fn portrange_contains(this: range::PortRange, port: u16) -> bool {
    this.contains(port)
}


pub fn portrange_overlaps(this: range::PortRange, other: range::PortRange) -> bool {
    this.overlaps(&other)
}
