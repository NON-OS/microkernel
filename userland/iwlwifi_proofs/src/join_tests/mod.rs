// NONOS Operating System (AGPL-3.0-or-later)
//! The join against the modeled firmware and a scripted access point: WPA2
//! and WPA3-SAE (hash-to-element and hunting and pecking) run to an open port
//! with every firmware command in Linux's order and the keys the access point
//! derived in the firmware's table; data crosses both ways through the
//! net_core link protocol, decrypted by the firmware or by the station; every
//! refusal, silence and firmware failure ends the join with its own reason
//! and takes the contexts down exactly as far as they went; nothing but
//! authentication, association and EAPOL in the clear leaves before the
//! port opens; and leaving, the access point's deauthentication and a group
//! rekey each do what they must.

mod address;
mod data;
mod drive;
mod gate;
mod held;
mod inbox;
mod keys;
mod krack;
mod krack_group;
mod krack_rig;
mod leave;
mod order;
mod queues;
mod refusals;
mod rekey;
mod rig;
mod setup;
mod silences;
