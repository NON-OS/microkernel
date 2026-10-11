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

//! The rig: the modeled device and its responder.

use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::firmware_region_sizes;
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::txq::TX_REGION;
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{reply, Mem, Model, Seen, TxModel};

/// The modeled device, booted, with its transmit region.
pub struct Rig {
    pub model: Model,
    pub tx: Rc<Mem>,
    pub ap: Rc<RefCell<Ap>>,
    pub(super) fw_regions: Vec<Rc<Mem>>,
}

pub fn rig(sec: Security, sw: Switches) -> Rig {
    rig_with(sec, sw, SessionReply::Start)
}

pub fn rig_with(sec: Security, sw: Switches, session: SessionReply) -> Rig {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let ctrl = Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000);
    let rbs = Mem::new(RB_REGION, 0x2000_0000);
    let sizes = firmware_region_sizes(&classify(SO_GF)).unwrap();
    let fw_regions =
        sizes.iter().enumerate().map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    let model = Model::new(ctrl, rbs);
    let tx = Mem::new(TX_REGION, TX_DEV);
    let ap = Rc::new(RefCell::new(Ap::new(sec, sw)));
    let mut t = TxModel::new(tx.clone());
    let air_ap = ap.clone();
    t.air = Some(Box::new(move |f: &[u8]| air_ap.borrow_mut().on_tx(f)));
    model.s.borrow_mut().txq = Some(t);
    model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| {
        let action = c.payload.get(4).copied().unwrap_or(0);
        match (c.group, c.cmd, action) {
            (0x3, 0x05, 1) => match session {
                SessionReply::Start => vec![reply(c, vec![]), session_notif(1, 1)],
                SessionReply::Refuse => vec![reply(c, vec![]), session_notif(0, 0)],
                SessionReply::Never => vec![reply(c, vec![])],
            },
            _ => vec![reply(c, vec![])],
        }
    }));
    Rig { model, tx, ap, fw_regions }
}
