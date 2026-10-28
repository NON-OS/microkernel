/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The link's keys into the chip (`vif::key`), as mac80211 hands them to
//! Linux's `mt7921_set_key` once the handshake is done: the pairwise key
//! under the access point's entry, which must be there; a group key, with
//! the IGTK, under the interface's own, which association pointed at the
//! BSS. Each command's copy is wiped once the firmware answered. A line
//! says which key went in or was refused, and nothing of the key.

use nonos_wifi_core::softmac::ChipKey;

use super::Mt7921;
use crate::mcu::cmd::STA_REC_UPDATE;
use crate::say::Line;
use crate::vif::key::{group, pairwise};

impl Mt7921 {
    pub(super) fn put_key(&mut self, k: &ChipKey<'_>) -> bool {
        if self.dead {
            return false;
        }
        let (mut body, what) = match k {
            ChipKey::Pairwise(tk) if self.peer.is_some() => (pairwise(tk), "pairwise"),
            ChipKey::Group { id, gtk, igtk } if self.joined.is_some() => {
                (group(*id, gtk, igtk.map(|i| i.1)), "group")
            }
            _ => return false,
        };
        let ok = self.card.secret(STA_REC_UPDATE, &mut body);
        self.note(ok);
        if matches!(k, ChipKey::Pairwise(_)) {
            self.card.keyed = ok;
        }
        let said = if ok { " key in the chip" } else { " key refused by the firmware" };
        Line::new(what).text(said).say();
        ok
    }
}
