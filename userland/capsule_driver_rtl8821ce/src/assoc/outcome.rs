// NONOS Operating System (AGPL-3.0-or-later)

//! How a join ended.

use alloc::boxed::Box;

use nonos_wifi_core::mlme::{Mlme, MlmeState};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::Supplicant;

/// How a join attempt ended. The supplicant is boxed so the bare endings stay
/// small.
pub enum Outcome {
    /// Associated: the pairwise and group keys and the AP to install them for,
    /// the AKM that ran, and the supplicant the data path keeps answering the
    /// AP's group key handshakes with.
    Joined {
        bssid: [u8; 6],
        channel: u8,
        ptk: [u8; 16],
        gtk: [u8; 16],
        gtk_id: u8,
        akm: Akm,
        supplicant: Box<Supplicant>,
    },
    /// The association was refused or the handshake broke; the report's
    /// `failure` says where.
    Refused,
    /// The AP stopped responding before the join completed.
    TimedOut,
}

/// How the join ended, if it has: connected (with what it installed) or
/// refused.
pub(super) fn ended(mlme: &Mlme) -> Option<Outcome> {
    match mlme.state() {
        MlmeState::Connected => Some(finish(mlme)),
        MlmeState::Failed => Some(Outcome::Refused),
        _ => None,
    }
}

/// How a join that ran out of time ended: connected if it got there.
pub(super) fn out_of_time(mlme: &Mlme) -> Outcome {
    if mlme.state() == MlmeState::Connected {
        return finish(mlme);
    }
    Outcome::TimedOut
}

// Pull the negotiated keys out of a connected machine.
pub(super) fn finish(mlme: &Mlme) -> Outcome {
    let (Some(tk), Some(gtk), Some(gtk_id), Some(akm), Some(supplicant)) =
        (mlme.tk(), mlme.gtk(), mlme.gtk_id(), mlme.akm(), mlme.supplicant())
    else {
        return Outcome::Refused;
    };
    let mut ptk = [0u8; 16];
    let mut group = [0u8; 16];
    if tk.len() < 16 || gtk.len() < 16 {
        return Outcome::Refused;
    }
    ptk.copy_from_slice(&tk[..16]);
    group.copy_from_slice(&gtk[..16]);
    Outcome::Joined {
        bssid: mlme.bssid(),
        channel: mlme.channel(),
        ptk,
        gtk: group,
        gtk_id,
        akm,
        supplicant: Box::new(supplicant),
    }
}
