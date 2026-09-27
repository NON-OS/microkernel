// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/uefi/types/attributes.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/uefi/types/attributes.rs"]
pub mod attributes;

pub fn variableattributes_empty() -> attributes::VariableAttributes {
    attributes::VariableAttributes::empty()
}

pub fn variableattributes_bits(this: attributes::VariableAttributes) -> u32 {
    this.bits()
}

pub fn variableattributes_from_bits(bits: u32) -> attributes::VariableAttributes {
    attributes::VariableAttributes::from_bits(bits)
}

pub fn variableattributes_from_bits_truncate(bits: u32) -> attributes::VariableAttributes {
    attributes::VariableAttributes::from_bits_truncate(bits)
}

pub fn variableattributes_contains(this: attributes::VariableAttributes, other: attributes::VariableAttributes) -> bool {
    this.contains(other)
}

pub fn variableattributes_is_empty(this: attributes::VariableAttributes) -> bool {
    this.is_empty()
}

pub fn variableattributes_is_non_volatile(this: attributes::VariableAttributes) -> bool {
    this.is_non_volatile()
}

pub fn variableattributes_is_runtime_access(this: attributes::VariableAttributes) -> bool {
    this.is_runtime_access()
}

pub fn variableattributes_requires_authentication(this: attributes::VariableAttributes) -> bool {
    this.requires_authentication()
}

pub fn variableattributes_intersection(this: attributes::VariableAttributes, other: attributes::VariableAttributes) -> attributes::VariableAttributes {
    this.intersection(other)
}

pub fn variableattributes_union(this: attributes::VariableAttributes, other: attributes::VariableAttributes) -> attributes::VariableAttributes {
    this.union(other)
}

