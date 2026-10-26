// NONOS Operating System (AGPL-3.0-or-later)

//! Proofs for the association radio glue (`assoc`): a management frame is
//! classified as management, an EAPOL-carrying data frame is unwrapped to its
//! payload, anything else is ignored, an EAPOL payload wraps into an 802.11 data
//! frame addressed to the AP, and a join against a silent radio times out rather
//! than hanging.

mod beacons;
mod budget;
mod deadline;
mod dedup;
mod facts;
mod frames;
mod handshake;
mod idle;
mod keys;
mod policy;
mod radio;
mod resend;
mod silent;
