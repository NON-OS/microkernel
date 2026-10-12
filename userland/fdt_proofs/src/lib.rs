// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the kernel's device tree parser. `arch::fdt` is the
//! kernel's own source mounted by `#[path]`, so the `crate::arch::fdt` paths
//! inside it resolve here exactly as they do in the kernel.

pub mod arch;

#[cfg(test)]
mod blobs;
#[cfg(test)]
mod builder;
#[cfg(test)]
mod exercise;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod msi_tree;
#[cfg(test)]
mod uart_tree;

#[cfg(test)]
mod chosen_tests;
#[cfg(test)]
mod cpu_release_tests;
#[cfg(test)]
mod header_tests;
#[cfg(test)]
mod intx_hostile_tests;
#[cfg(test)]
mod intx_tests;
#[cfg(test)]
mod irq_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod mailbox_tests;
#[cfg(test)]
mod msi_hostile_tests;
#[cfg(test)]
mod msi_refusal_tests;
#[cfg(test)]
mod msi_tests;
#[cfg(test)]
mod mutate_tests;
#[cfg(test)]
mod node_tests;
#[cfg(test)]
mod pci_tests;
#[cfg(test)]
mod platform_hostile_tests;
#[cfg(test)]
mod platform_tests;
#[cfg(test)]
mod reserve_tests;
#[cfg(test)]
mod psci_tests;
#[cfg(test)]
mod rpi4_platform_tests;
#[cfg(test)]
mod rpi4_tests;
#[cfg(test)]
mod seed_erase_tests;
#[cfg(test)]
mod smmu_irq_tests;
#[cfg(test)]
mod smmu_tests;
#[cfg(test)]
mod structure_tests;
#[cfg(test)]
mod translate_tests;
#[cfg(test)]
mod uart_tests;
#[cfg(test)]
mod virt_tests;
#[cfg(test)]
mod virt_variant_tests;
