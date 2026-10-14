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

//! The first bring-up: the card named, its firmware chosen and parsed, the
//! regions mapped and kept for the capsule's life, then booted and set up.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::driver::Driver;
use crate::firmware::gen3::classify;
use crate::firmware::gen3::dev::Dev;
use crate::firmware::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::plan::control_len_pnvm;
use crate::firmware::gen3::pnvm::reserve as pnvm_reserve;
use crate::firmware::gen3::select::{mac_type, rf_type, Refusal};
use crate::firmware::gen3::ucode::Ucode;
use crate::firmware::{gen3_blob, gen3_pnvm};
use crate::regs::Regs;

use super::super::grant::Grant;
use super::super::say::Line;
use super::alive::boot_alive;
use super::finish::finish;
use super::identify::identify;
use super::map::map_all;
use super::report::station_addr;
use super::seen::{Seen, Up};

pub fn bring_up(d: &Driver, ids: &mut Seen) -> Result<Up, Failure> {
    let (image, transport) = identify(d, ids)?;
    let blob = gen3_blob(image).ok_or(Failure::Refused(Refusal::ImageNotBundled(image)))?;
    Line::new().text(b"firmware ").text(blob.name.as_bytes()).send();
    let ucode = Ucode::parse(blob.bytes).ok_or(Failure::Image)?;
    let fw = classify(blob.bytes);
    let sizes = firmware_region_sizes(&fw).ok_or(Failure::Image)?;
    // Space after the image loader for the largest platform NVM section this
    // adapter could be given; the SKU that picks one is only known at ALIVE.
    let pnvm = gen3_pnvm(image);
    let reserve = pnvm.map_or(0, |f| pnvm_reserve(f, mac_type(ids.hw_rev), rf_type(ids.rf_id)));
    let ctrl_len = control_len_pnvm(ucode.iml.len(), reserve).ok_or(Failure::Image)?;
    let grants = map_all(d, ctrl_len, &sizes).ok_or(Failure::NoDma)?;

    // From the kick on the device holds these addresses: the grants live as
    // long as the capsule does.
    let grants: &'static [Grant] = Vec::leak(grants);
    let (ctrl, rest) = grants.split_first().ok_or(Failure::NoDma)?;
    let (rbs, fw_grants) = rest.split_first().ok_or(Failure::NoDma)?;
    let fw_refs: &'static [&'static Grant] = Vec::leak(fw_grants.iter().collect());
    let regs: &'static Regs = Box::leak(Box::new(d.regs));
    let mut dev = Dev::new(regs, ctrl, rbs);
    let mem = Memory { ctrl, rbs, fw: fw_refs };
    let board =
        Board { hw_rev: ids.hw_rev, imr_enabled: transport.imr_enabled, rf_id: ids.rf_id, pnvm };
    boot_alive(&mut dev, &mem, &fw, &ucode, &board)?;

    finish(dev, &ucode, &transport, station_addr())
}
