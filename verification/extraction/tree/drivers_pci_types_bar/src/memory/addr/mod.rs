// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../src/memory/addr/phys.rs"]
pub mod phys;

#[path = "../../../../../../../src/memory/addr/virt.rs"]
pub mod virt;

pub use phys::PhysAddr;
pub use virt::VirtAddr;
