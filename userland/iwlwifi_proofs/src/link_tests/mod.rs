// NONOS Operating System (AGPL-3.0-or-later)

//! Integration proof for the iwlwifi data path: the shared `LinkStation` framing
//! engine composed with the gen3 transmit command and RX-MPDU extraction. The
//! unit proofs check each piece; this checks they fit together, so an ethernet
//! frame becomes a firmware TX command on transmit and a received notification
//! becomes an ethernet frame on receive. Intel does CCMP in firmware, so the
//! station frames plaintext (`Ccmp::Hardware`) and the radio encrypts. The only
//! part not exercised here is the DMA ring itself, which is the on-silicon step.

mod frames;
mod receive;
mod transmit;
