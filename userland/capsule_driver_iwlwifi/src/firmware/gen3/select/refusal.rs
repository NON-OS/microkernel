// NONOS Operating System (AGPL-3.0-or-later)

//! Why an adapter does not get a gen3 firmware, its phrase and detail word.

use super::Image;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// The PCI device is not one of the platforms this driver boots (the
    /// AX210 family's SO, TY and MA, and the BZ family's Gl and Bz).
    NotSoDevice(u16),
    /// The MAC is not one this driver has a firmware name for, or not one
    /// of its PCI device's family.
    MacNotBundled(u16),
    /// A MAC step whose firmware is not carried (Meteor Lake other than B,
    /// Gl other than C, Bz other than B).
    MacStepNotBundled(u8),
    /// The image this adapter runs is known but not in the tree.
    ImageNotBundled(Image),
    /// The RF type has no bundled firmware on this MAC (JF, or a blank OTP).
    RfNotBundled(u16),
    /// A dual-radio (CDB) module needs a gf4 or fm4 image, not bundled.
    CdbNotBundled,
    /// An FM radio of a step other than C (Linux would load an fm-b0 image).
    RfStepNotBundled(u8),
}

impl Refusal {
    /// One phrase for the boot console.
    pub fn text(&self) -> &'static str {
        match self {
            Refusal::NotSoDevice(_) => "not a platform this driver has firmware for",
            Refusal::MacNotBundled(_) => "no bundled firmware for this MAC type",
            Refusal::RfNotBundled(_) => "no bundled firmware for this RF module",
            Refusal::CdbNotBundled => "a dual-radio module, whose firmware is not bundled",
            Refusal::MacStepNotBundled(_) => "a MAC step whose firmware is not bundled",
            Refusal::ImageNotBundled(_) => "its firmware is known but not in this build",
            Refusal::RfStepNotBundled(_) => "an RF step whose firmware is not bundled",
        }
    }

    /// The detail word: the kind in bits 16-19, what was refused below.
    pub fn code(&self) -> u32 {
        match *self {
            Refusal::NotSoDevice(id) => 0x1_0000 | u32::from(id),
            Refusal::MacNotBundled(mac) => 0x2_0000 | u32::from(mac),
            Refusal::RfNotBundled(rf) => 0x3_0000 | u32::from(rf),
            Refusal::CdbNotBundled => 0x4_0000,
            Refusal::MacStepNotBundled(step) => 0x5_0000 | u32::from(step),
            Refusal::ImageNotBundled(image) => 0x6_0000 | image as u32,
            Refusal::RfStepNotBundled(step) => 0x7_0000 | u32::from(step),
        }
    }
}
