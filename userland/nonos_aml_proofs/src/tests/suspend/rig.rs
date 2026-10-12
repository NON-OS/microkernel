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

//! A laptop's sleep objects as ACPI 6.5 lays them out (7.4, 7.3.13, 9.1):
//! \_S0 and \_S3, \_TTS, \_PTS, \_WAK and \_SI._SST; a USB controller that
//! wakes from S3 through a power resource and _DSW, a network card that
//! has only _PSW, a keyboard whose _PRW reaches only S1, a device with no
//! _PRW, and a sleep button the policy does not name.

use crate::consts::at;
use crate::enc::*;
use crate::rig::Rig;

pub fn sx(a: u64, b: u64) -> Vec<u8> {
    package(4, &[int(a), int(b), int(0), int(0)])
}

fn prw(gpe: u64, state: u64, resources: &[&str]) -> Vec<u8> {
    let mut els = vec![int(gpe), int(state)];
    els.extend(resources.iter().map(|r| name(r)));
    name_obj("_PRW", &package(els.len() as u8, &els))
}

pub fn laptop(with_s3: bool) -> Rig {
    let nop = |n: &str, args: u8| method(n, args, &[]);
    let pwr = power_res(
        "\\_SB.PWRR",
        0,
        0,
        &cat(&[&method("_STA", 0, &ret(&int(1))), &nop("_ON", 0), &nop("_OFF", 0)]),
    );
    let devices = cat(&[
        &device("\\_SB", &[]),
        &pwr,
        &device("\\_SB.XHCI", &cat(&[&prw(0x6D, 3, &["\\_SB.PWRR"]), &nop("_DSW", 3)])),
        &device("\\_SB.LAN0", &cat(&[&prw(0x6E, 4, &[]), &nop("_PSW", 1)])),
        &device("\\_SB.KBD0", &cat(&[&prw(0x6F, 1, &[]), &nop("_PSW", 1)])),
        &device("\\_SB.NONE", &[]),
        &device("\\_SB.SLPB", &cat(&[&prw(0x70, 4, &[]), &nop("_PSW", 1)])),
    ]);
    let sleep = cat(&[
        &name_obj("\\_S0", &sx(0, 0)),
        &if with_s3 { name_obj("\\_S3", &sx(5, 5)) } else { Vec::new() },
        &nop("\\_TTS", 1),
        &nop("\\_PTS", 1),
        &method("\\_WAK", 1, &ret(&package(2, &[int(0), int(0)]))),
        &device("\\_SI", &nop("_SST", 1)),
    ]);
    Rig::new(&cat(&[&devices, &sleep])).connected()
}

pub const VECTOR: u64 = 0x9A000;

pub fn policy(r: &Rig) -> Vec<crate::namespace::Id> {
    ["\\_SB.XHCI", "\\_SB.LAN0", "\\_SB.KBD0", "\\_SB.NONE"]
        .iter()
        .map(|p| at(&r.aml, p).unwrap())
        .collect()
}
