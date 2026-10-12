// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 kernel's self-relocation.

#[cfg(test)]
#[path = "../../../src/arch/aarch64/boot/relocate"]
mod relocate {
    pub(crate) mod apply;
    pub(crate) mod rela;
}

#[cfg(test)]
mod elf;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod image_tests;
#[cfg(test)]
mod loaded;
