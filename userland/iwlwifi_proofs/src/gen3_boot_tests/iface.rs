// NONOS Operating System (AGPL-3.0-or-later)

//! The interface is created at the station address it is given.

use super::answers::firmware_answers;
use super::rig::{board, polls, rig, so, SO_GF};
use crate::gen3::bringup::boot;
use crate::gen3::cmds::*;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::Memory;
use crate::gen3::ucode::Ucode;
use crate::gen3::up::up;
use crate::gen3_model::Mem;

/// The interface takes the address bring-up drew for this boot, in both the
/// MAC and the link context.
#[test]
fn the_interface_is_created_at_the_station_address_it_is_given() {
    let r = rig();
    let ucode = Ucode::parse(SO_GF).unwrap();
    r.model.s.borrow_mut().responder = Some(firmware_answers(ucode.capa(CAPA_MCC_UPDATE_11AX_SUPPORT)));
    let mut c = polls();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board()).expect("alive");
    let drawn = [0x5E, 0x11, 0x22, 0x33, 0x44, 0x55];
    up(&mut dev, &mut c, &ucode, &so(), drawn).expect("up");
    let cmds = r.model.s.borrow().commands.clone();
    let mac = cmds.iter().find(|s| (s.group, s.cmd) == (MAC_CONF_GROUP, MAC_CONFIG_CMD)).unwrap();
    let link = cmds.iter().find(|s| (s.group, s.cmd) == (MAC_CONF_GROUP, LINK_CONFIG_CMD)).unwrap();
    assert_eq!(&mac.payload[12..18], &drawn);
    assert_eq!(&link.payload[16..22], &drawn);
}
