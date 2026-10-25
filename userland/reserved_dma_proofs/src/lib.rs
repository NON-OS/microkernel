// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for reserved DMA ranges. The kernel's files are mounted at
//! the top, side by side so their `super::` paths resolve; `memory` and
//! `arch` give the rest the `crate::` paths they name types by.

#![allow(dead_code)]

#[path = "../../../src/memory/iommu/reserved/admit.rs"]
pub mod admit;
#[path = "../../../src/memory/iommu/reserved/answer.rs"]
pub mod answer;
#[path = "../../../src/hardware/broker/confine/avoid.rs"]
pub mod avoid;
#[path = "../../../src/memory/iommu/device.rs"]
pub mod device;
#[path = "../../../src/arch/fdt/find/reserved_dma/entry.rs"]
pub mod entry;
#[path = "../../../src/hardware/broker/confine/held.rs"]
pub mod held;
#[path = "../../../src/arch/fdt/find/iommus/limits.rs"]
pub mod iommus_limits;
#[path = "../../../src/hardware/broker/confine/iova_space.rs"]
pub mod iova_space;
#[path = "../../../src/arch/aarch64/iommu/reserved/lookup.rs"]
pub mod lookup;
#[path = "../../../src/memory/iommu/reserved/range.rs"]
pub mod range;
#[path = "../../../src/memory/iommu/reserved/refusal.rs"]
pub mod refusal;

pub mod memory {
    pub mod iommu {
        pub use crate::device::DeviceAddress;
        pub mod reserved {
            pub use crate::admit::admit;
            pub use crate::answer::{Reserved, MAX_RESERVED};
            pub use crate::range::ReservedRange;
            pub use crate::refusal::Refusal;
        }
    }
}
pub mod arch {
    pub mod fdt {
        pub mod find {
            pub mod iommus {
                pub use crate::iommus_limits::MAX_STREAMS;
            }
            pub mod reserved_dma {
                pub use crate::entry::{Owner, TreeRange, MAX_STREAMS};
            }
        }
    }
}

#[cfg(test)]
mod admit_tests;
#[cfg(test)]
mod avoid_tests;
#[cfg(test)]
mod edge_tests;
#[cfg(test)]
mod held_tests;
#[cfg(test)]
mod lookup_tests;
#[cfg(test)]
mod order_tests;
