// NONOS Operating System (AGPL-3.0-or-later)

//! The BZ family (Intel Wi-Fi 7 BE200, "Gl", 8086:272B, and the BE201 CNVi,
//! "Bz", 8086:A840, 7740 and 4D40) on the gen3 path, host-proven only: the
//! real linux-firmware images (API 92, linux-firmware bf992df7) parse and
//! speak every layout the driver encodes; the boot runs the V2 CSR sequence
//! against a modeled device that answers V2 bits only; selection, the Bz
//! MAC step, the receive and transmit layouts and the two command layouts
//! whose versions moved are pinned. No BE200 or BE201 has booted this.

mod boot;
mod cmds;
mod firmware;
mod layouts;
mod mutants;
mod rig;
mod select;
mod step;
