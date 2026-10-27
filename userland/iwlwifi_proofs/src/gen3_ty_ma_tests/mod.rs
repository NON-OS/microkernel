// NONOS Operating System (AGPL-3.0-or-later)

//! The discrete AX210 (TY, 8086:2725) and Meteor Lake (MA, 8086:7E40 and
//! 2729) on the gen3 path, host-proven only: the real linux-firmware images
//! (API 86, linux-firmware bf992df7) parse with the driver's own parser and
//! speak every version the driver encodes, their PNVM files give each its
//! own section, the discrete AX210 boots with its boot LTR, and a dual-radio
//! (GF4) module puts a 5 GHz PHY on LMAC 1. No TY or MA part has booted it.

mod boot;
mod carried;
mod differs;
mod firmware;
mod rig;
