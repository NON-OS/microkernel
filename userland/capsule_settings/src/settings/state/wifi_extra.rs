/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi panel's view of the driver's link and of the saved networks.
//!
//! Saved networks are held here by name only: the passphrases stay sealed in
//! the store, and a list opened to read the names is wiped as it drops.

use nonos_wifi_client::{Driver, Link, SavedError, ScanNetwork, SLOTS};

pub struct WifiExtra {
    /// The Wi-Fi driver service registered, if any.
    pub driver: Option<Driver>,
    /// Its link as it last reported it.
    pub link: Option<Link>,
    /// Whether a network joined from this panel is remembered. Off until the
    /// person turns it on: nothing is kept unless asked for.
    pub remember: bool,
    /// Whether this boot keeps state, which remembering needs.
    pub keeps: bool,
    pub saved: [ScanNetwork; SLOTS],
    pub saved_count: usize,
    /// Why the saved networks could not be read, when they could not.
    pub saved_err: Option<SavedError>,
    /// What the last remember or forget did, for the panel to say.
    pub notice: Option<&'static str>,
}

impl WifiExtra {
    pub const fn new() -> Self {
        Self {
            driver: None,
            link: None,
            remember: false,
            keeps: false,
            saved: [ScanNetwork::EMPTY; SLOTS],
            saved_count: 0,
            saved_err: None,
            notice: None,
        }
    }

    /// The name of the network the radio is associated with, if it is.
    pub fn joined(&self) -> Option<&[u8]> {
        self.link.as_ref().filter(|l| l.associated).map(|l| l.ssid())
    }

    pub fn is_saved(&self, ssid: &[u8]) -> bool {
        self.saved[..self.saved_count].iter().any(|n| n.ssid() == ssid)
    }
}
