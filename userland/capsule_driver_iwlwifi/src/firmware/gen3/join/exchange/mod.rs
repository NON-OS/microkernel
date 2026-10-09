// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The air side of a join: the shared MLME (`nonos_wifi_core::mlme`) owns
//! the protocol and the keys (Open System or SAE authentication, the
//! association, the four-way handshake through the supplicant); this feeds
//! it what the firmware received and sends what it returns.
//!
//! A management frame is the MLME's whole. A data frame counts only as the
//! access point's EAPOL to this station in the clear (FromDS, transmitter the
//! BSSID, receiver this station, not protected, LLC/SNAP with EtherType
//! 0x888E right after its header); its payload goes to the MLME, and the
//! EAPOL reply goes back in an unprotected data frame. Nothing else is sent
//! before the port opens: management frames only on the management queue,
//! and on the data queue only those EAPOL replies.
//!
//! The MLME sends each frame once. A frame left unanswered is sent again
//! each time `RETX_MS` passes on the clock without the exchange moving, at
//! most `IDLE_TRIES` times in a row; frames that do not move it (beacons,
//! other stations' traffic) neither delay a resend nor use up the exchange.
//! When the state moves on without a reply to send, the access point is to
//! speak next and nothing is resent. The whole exchange is held to
//! `EXCHANGE_MS` on the clock. When the association response moves the MLME to
//! the four-way handshake, the firmware is told before any EAPOL frame is
//! answered (`Bss::associated`). A session protection that ends before the
//! port opens is asked for again.

mod air;
mod assoc;
mod drive;
mod eapol;
mod end;
mod entry;
mod feed;
pub mod limits;
mod progress;
mod step;

pub use end::ExchangeEnd;
pub use entry::exchange;
pub use progress::{state_code, Progress};
