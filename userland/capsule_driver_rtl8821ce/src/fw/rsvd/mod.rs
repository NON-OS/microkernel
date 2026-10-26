// NONOS Operating System (AGPL-3.0-or-later)

//! Stage one firmware chunk into the on-chip packet buffer as a reserved page.
//! This is the hardware-specific half of firmware download: the DDMA channel can
//! only copy out of the packet buffer, so each chunk first travels through the
//! beacon queue into reserved-page 0. The engine lays the 48-byte TX descriptor
//! and the payload into a DMA buffer, publishes a beacon buffer descriptor, arms
//! the beacon-valid status, kicks the beacon queue and waits for the card to
//! report the page landed, then returns the DDMA source address the caller
//! copies from. The register sequence follows rtw88 `rtw_fw_write_data_rsvd_page`
//! (the 11ac path) and `rtw_pci_write_data_rsvd_page` in `pci.c`. The exact byte
//! layout and register program are checked against a modeled device in
//! `rtl8821ce_proofs`; only the card actually consuming the beacon page is left
//! for on-silicon bring-up.

mod stage;
mod valid;

pub use stage::{stage_at, stage_chunk};
