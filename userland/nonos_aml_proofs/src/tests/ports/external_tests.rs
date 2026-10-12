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

//! Root ports' external-facing and DMA properties read from _DSD under
//! their own UUIDs; a port that declares neither is internal, and a
//! property under the wrong UUID is not believed.

use crate::enc::*;
use crate::ports::{facing, Facing, DMA_PROPERTY_UUID, EXTERNAL_FACING_UUID};
use crate::rig::Rig;
use crate::serial::DEVICE_PROPERTIES_UUID;
use crate::tests::platform::run::evaluator;

fn dsd(uuid: &[u8; 16], name: &str, v: u64) -> Vec<u8> {
    let prop = package(2, &[string(name), int(v)]);
    name_obj("_DSD", &package(2, &[buffer(16, uuid), package(1, &[prop])]))
}

fn rig() -> Rig {
    Rig::new(&cat(&[
        &device("\\_SB.PCI0", &[]),
        &device("\\_SB.PCI0.RP05", &dsd(&EXTERNAL_FACING_UUID, "ExternalFacingPort", 1)),
        &device("\\_SB.PCI0.RP01", &dsd(&DMA_PROPERTY_UUID, "DmaProperty", 1)),
        &device("\\_SB.PCI0.RP02", &[]),
        &device("\\_SB.PCI0.RP03", &dsd(&DEVICE_PROPERTIES_UUID, "ExternalFacingPort", 1)),
        &device("\\_SB.PCI0.RP04", &dsd(&EXTERNAL_FACING_UUID, "ExternalFacingPort", 0)),
    ]))
    .connected()
}

#[test]
fn each_root_port_as_it_declares_itself() {
    let mut r = rig();
    let got: Vec<Facing> = ["RP05", "RP01", "RP02", "RP03", "RP04"]
        .iter()
        .map(|p| {
            let id = crate::consts::at(&r.aml, &format!("\\_SB.PCI0.{p}")).unwrap();
            let Rig { aml, probe } = &mut r;
            facing(aml, &mut evaluator(probe), id)
        })
        .collect();
    let f = |external, dma_property| Facing { external, dma_property };
    assert_eq!(
        got,
        [f(true, None), f(false, Some(1)), f(false, None), f(false, None), f(false, None)]
    );
}
