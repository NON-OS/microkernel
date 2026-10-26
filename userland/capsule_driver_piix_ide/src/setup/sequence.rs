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

//! The bring-up: claim, command bits and port grants; the DMA grant; both
//! engines stopped in case firmware left one running; then each channel
//! reset with its interrupt held down, and each drive slot probed. A
//! failing step gives back every grant and the claim. A slot holding a
//! packet device or a disk the driver cannot serve is named and left alone.

use super::{claim, dma, driver::Driver};
use crate::channel::{Channel, Probe};
use crate::constants::ata::IDENTIFY_WORDS;
use crate::constants::bmide::{bmis_drive_dma, BMIC, BMIS};
use crate::constants::pci::CHANNELS;
use crate::discover::Found;
use crate::disk::{parse, unit_of, Disk, UNITS};
use crate::regs::Port;
use crate::say;

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let grants = claim::claim(&dev)?;
    let Some(dma) = dma::map(dev.device_id, grants.epoch) else {
        grants.release();
        return Err("dma map failed");
    };
    let bm = Port::new(grants.bm);
    let channels: [Option<Channel>; CHANNELS] = core::array::from_fn(|c| {
        grants.channels[c]
            .map(|g| Channel::new(c, Port::new(g.cmd), Port::new(g.ctl), g.ctl_ports, bm))
    });
    for ch in channels.iter().flatten() {
        ch.bm_write8(BMIC, 0);
    }
    let mut disks: [Option<Disk>; UNITS] = [None; UNITS];
    for (c, ch) in channels.iter().enumerate() {
        let Some(ch) = ch else { continue };
        if !ch.reset() {
            continue;
        }
        let mut dma_bits = ch.bm_read8(BMIS) & (bmis_drive_dma(0) | bmis_drive_dma(1));
        for drive in 0..2u8 {
            let unit = unit_of(c, drive);
            let mut words = [0u16; IDENTIFY_WORDS];
            match ch.probe(drive, &mut words) {
                Probe::Absent => {}
                Probe::Packet => say::packet(unit),
                Probe::Ata => match parse(&words) {
                    Ok(id) => {
                        disks[unit] = Some(Disk { channel: c, drive, id });
                        dma_bits |= bmis_drive_dma(drive);
                    }
                    Err(why) => say::refused(unit, why),
                },
            }
        }
        // The bits firmware sets for each drive it set up for DMA; the
        // latched error and interrupt are written as zero, which keeps them.
        ch.bm_write8(BMIS, dma_bits);
    }
    Ok(Driver { grants, dma, channels, disks })
}
