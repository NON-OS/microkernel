// NONOS Operating System (AGPL-3.0-or-later)

//! Proofs that the RTL8821CE `RtlLink` satisfies the shared `LinkPort` contract
//! net_core drives. An unassociated link reports down, offers no MAC and refuses
//! to transmit; once associated it reports its MAC and link up. A transmitted
//! Ethernet frame is framed by the station and laid into the TX ring with the
//! hardware CCMP security type set, so the radio encrypts from the CAM, and the
//! write-index doorbell is rung. A received 802.11 MPDU is pulled off the RX ring
//! and parsed back to the exact Ethernet frame that produced it. Driven through
//! the real `nonos_wifi_core::netif::LinkPort` trait against modeled DMA and a
//! modeled card; only the card actually moving bytes is left for on-silicon
//! bring-up.
//!
//! The staged receive frame is what this chip hands up (rtw88 reads the same
//! layout): the access point's FromDS frame, Protected bit still set, decrypted
//! in place with the CCMP header and MIC left on, the 4-byte FCS after it, and
//! a descriptor whose security type says AES and whose SWDEC bit is clear. An
//! earlier version of this fixture staged an unprotected station-to-AP frame,
//! which the link now refuses as it must (`an_unprotected_frame_is_refused_*`).

pub(crate) mod card;
mod checks;
mod clear;
mod deauth;
mod group;
mod krack;
pub(crate) mod krack_rig;
mod pmf;
pub(crate) mod receive;
mod replay;
pub(crate) mod rig;
pub(crate) mod rxbuf;
mod stall;
mod up;
