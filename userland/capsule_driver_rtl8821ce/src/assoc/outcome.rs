// NONOS Operating System (AGPL-3.0-or-later)

//! How a join ended.

use alloc::boxed::Box;

use nonos_wifi_core::key::HeldKey;
use nonos_wifi_core::mlme::{Mlme, MlmeState};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::Supplicant;

/// How a join attempt ended (the supplicant boxed, so the endings stay small).
pub enum Outcome {
    /// Associated: the keys and the AP to install them for, the AKM that ran,
    /// and the supplicant that keeps answering the AP's group key handshakes.
    Joined {
        bssid: [u8; 6],
        channel: u8,
        ptk: HeldKey,
        gtk: HeldKey,
        gtk_id: u8,
        akm: Akm,
        supplicant: Option<Box<Supplicant>>,
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
    if tk.len() < 16 || gtk.len() < 16 {
        return Outcome::Refused;
    }
    // The keys are copied once, into holders that wipe them where they lie.
    let mut o = Outcome::Joined {
        bssid: mlme.bssid(),
        channel: mlme.channel(),
        ptk: HeldKey([0; 16]),
        gtk: HeldKey([0; 16]),
        gtk_id,
        akm,
        supplicant: Some(Box::new(supplicant)),
    };
    if let Outcome::Joined { ptk, gtk: group, .. } = &mut o {
        ptk.0.copy_from_slice(&tk[..16]);
        group.0.copy_from_slice(&gtk[..16]);
    }
    o
}
