/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! What a join's status code means, in a line a panel can show.
//!
//! Codes -1 to -12 are the drivers' (the RTL8821CE's serve/connect/result.rs
//! names each), and -13 their refusal of a rejoin once the person left;
//! -16 is this client's, for a join held to WPA3 because the saved list could
//! not be opened; -38 is how the iwlwifi driver refuses a join it cannot run;
//! the last is this client's own when no reply came.

use super::connect::NO_REPLY;
use super::services::CANNOT_JOIN;
use crate::saved::SAVED_UNREADABLE;

pub fn join_text(code: i32) -> &'static str {
    match code {
        0 => "Joined",
        -1 => "The radio is down or the request was malformed",
        -2 => "The network was not heard on any channel",
        -3 | -4 => "The keys could not be installed in the card",
        -5 => "The access point refused the association",
        /*
         * A wrong passphrase ends here: the access point drops a handshake
         * whose integrity check fails rather than saying why.
         */
        -6 => "The handshake did not finish; check the passphrase",
        -7 => "Saved as WPA3, but the network now offers only WPA2; not joined",
        -8 => "The network's security is not supported (open, TKIP or Enterprise)",
        -9 => "A passphrase is 8 to 63 characters, or 64 hex digits",
        -10 => "WPA3: the access point did not accept the password",
        -11 => "The handshake did not match the network's beacon; not joined",
        -12 => "No randomness for the handshake; not joined",
        -13 => "Not joined again: the network was left from Settings",
        -14 => "WPA3 could not start; try again",
        SAVED_UNREADABLE => "Saved networks not opened, so WPA3 only; this one offers WPA2",
        CANNOT_JOIN => "This driver cannot join networks yet",
        NO_REPLY => "The driver did not answer",
        _ => "The join failed",
    }
}
