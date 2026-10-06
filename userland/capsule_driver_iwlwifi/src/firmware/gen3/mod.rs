// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Gen3 (AX210-class, including Alder Lake CNVi) firmware self-load. Where the
//! legacy FH path streams each ucode section over a DMA channel, gen3 devices
//! read a context-information structure from host memory and load their own
//! firmware through an on-ROM image loader. This module owns the pieces of that
//! handoff: the context-information structure (`ctxt_info`), the boot control
//! registers (`csr`), and the register sequence that starts the load (`boot`).
//!
//! On top of those: firmware selection by MAC and RF type (`select`), the
//! firmware description (`ucode`), the DMA plan across broker-sized regions
//! (`plan`, `layout`), peripheral register access (`prph`), the receive and
//! command queues (`rxq`, `packet`, `cmdq`, `dev`), the start sequence and
//! ALIVE (`start`, `bringup`, `alive`), the post-ALIVE commands and NVM
//! (`cmds`, `nvm`, `up`), the passive scan request and its frames (`scan`,
//! `rx_frame`), the background sweep the serving loop pumps (`sweep`) and
//! what it hears (`heard`), how a failed bring-up is reported (`outcome`),
//! and joining a network (`station`, `txq`, `tx_cmd`, `tx_resp`, `rx_data`,
//! `join`). All of it
//! runs against the `Mmio`, `Region` and `Clock` traits, so the proofs drive
//! the whole sequence against a modeled device; the capsule supplies broker
//! grants and the uptime clock.

pub mod alive;
pub mod boot;
pub mod bringup;
pub mod cmdq;
pub mod cmds;
pub mod csr;
pub mod ctxt_info;
pub mod dev;
pub mod dram_map;
pub mod heard;
pub mod image;
pub mod join;
pub mod layout;
pub mod nvm;
pub mod outcome;
pub mod packet;
pub mod plan;
pub mod pnvm;
pub mod prph;
pub mod prph_scratch;
pub mod region;
pub mod regs;
pub mod rx_data;
pub mod rx_frame;
pub mod rx_mpdu;
pub mod rxq;
pub mod scan;
pub mod select;
pub mod start;
pub mod station;
pub mod sweep;
pub mod ucode;
pub mod up;

pub use boot::kick;
pub use ctxt_info::{CtxtInfoGen3, CTXT_INFO_GEN3_SIZE};
pub use dram_map::{classify, FwLayout};
pub use image::find_iml;
pub use prph_scratch::{DramImage, PrphScratch, PRPH_SCRATCH_SIZE};
pub use rx_mpdu::extract_mpdu;

pub mod tx_cmd;
pub mod tx_resp;
pub mod txq;

// Proven by the proofs, not run by the capsule: the single-region firmware
// staging `layout` replaced once a DMA grant was capped at 64 pages, the
// section walk under its re-exported name, and the transmit command's
// whole-frame form. They stay out of the capsule build so it stays
// warning-clean, as the RTL8821CE driver does with its calibration.
#[cfg(test)]
pub use dram_map::{stage, DramPlacement};
#[cfg(test)]
pub use image::{sections, Section};
#[cfg(test)]
pub use tx_cmd::{TX_CMD, TX_CMD_GEN3_HDR};
