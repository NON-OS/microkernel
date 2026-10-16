// NONOS Operating System (AGPL-3.0-or-later)

//! What a choice decides, and why it can be refused.

use crate::wpa::akm::Akm;

/// How the station will protect management frames.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pmf {
    /// Not negotiated.
    Off,
    /// Negotiated (MFPC set); the access point does not insist on it.
    Capable,
    /// Negotiated and required (MFPC and MFPR set).
    Required,
}

/// The security the join will run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Selection {
    pub akm: Akm,
    pub pmf: Pmf,
    /// For SAE: derive the password element by hash-to-element (the access
    /// point's RSNXE advertises it) rather than hunting and pecking.
    pub sae_h2e: bool,
}

/// Why a network cannot be joined as it advertises itself.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectError {
    /// The group or pairwise cipher is not CCMP-128.
    UnsupportedCipher,
    /// No offered AKM is one this station runs (802.1X/Enterprise, FT, OWE).
    UnsupportedAkm,
    /// The network was saved as WPA3 and now offers only WPA2.
    Downgrade,
    /// The capabilities contradict themselves (MFPR without MFPC) or name a
    /// management cipher other than BIP-CMAC-128 while requiring protection.
    BadProtection,
}
