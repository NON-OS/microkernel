/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Bring-up, in Linux v6.12 `mt7921_pci_probe` and `mt7921_init_hardware`
//! order: the chip handed to the driver, identified, the firmware package
//! read and checked, the subsystem reset, the rings set up, the firmware
//! downloaded and started, the MAC brought up, the interface added. Each
//! stage says itself on the console, and the first that fails is the
//! stage the panel shows; the card is then left quiet and still claimed.

mod package;
mod rings;
mod start;

use alloc::boxed::Box;

use crate::fw::package::{for_chip, File};
use crate::hw::{Mmio, Regs};
use crate::ids::Chip;
use crate::power::{identify, own};
use crate::radio::Mt7921;
use crate::say::{line, Line};
use crate::setup::Mapped;
use crate::stage::Stage;

/// Which package files a chip takes: the pinned ones (`for_chip`) in the
/// capsule; files the proofs make, with their own hashes, on the host.
pub type Pick = fn(Chip) -> (&'static File, &'static File);

pub fn run(m: &Mapped) -> (Option<Mt7921>, Stage) {
    // SAFETY: the broker mapped `len` bytes of BAR0 at `va` for this claim,
    // which this process holds for as long as it runs.
    run_with(Regs::new(Box::new(unsafe { Mmio::new(m.va, m.len) })), m, for_chip)
}

/// `run` over a register window and a choice of files given.
pub fn run_with(regs: Regs, m: &Mapped, pick: Pick) -> (Option<Mt7921>, Stage) {
    match up(regs, m, pick) {
        Ok(radio) => {
            line("stage ready: scanning");
            (Some(radio), Stage::Ready)
        }
        Err(stage) => {
            Line::new("stopped at stage ").dec(stage as u64).text(": ").text(stage.text()).say();
            (None, stage)
        }
    }
}

fn up(regs: Regs, m: &Mapped, pick: Pick) -> Result<Mt7921, Stage> {
    if !own::fw_own(&regs) || !own::drv_own(&regs) {
        let dead = regs.rr(crate::hw::addr::CONN_ON_LPCTL) == u32::MAX;
        return Err(if dead { Stage::DeadMmio } else { Stage::PowerFailed });
    }
    line("stage power: driver owns the card");
    let Some((chip, rev)) = identify(&regs) else {
        let dead = regs.rr(crate::hw::addr::HW_CHIPID) == u32::MAX;
        return Err(if dead { Stage::DeadMmio } else { Stage::NoAirPath });
    };
    Line::new("chip ").hex(chip.number()).text(" rev ").hex(rev.into()).say();
    let files = package::read(pick(chip))?;
    let mut card = rings::up(m, regs)?;
    let caps = start::firmware(&mut card, &files).inspect_err(|_| rings::quiet(&card, m))?;
    start::interface(card, m, chip, caps)
}
