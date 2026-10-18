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

//! The station side: its join state and a firmware handle over the rig.

use super::*;
use crate::gen3::bringup::boot;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::join::fw::Fw;
use crate::gen3::join::inbox::{Inbox, Queues};
use crate::gen3::layout::{Board, Memory};
use crate::gen3::txq::{TxQueue, TXQ_STRIDE};
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{Mem, Model, Polls};

/// The station's side of the rig: the transport, the queues and the inbox.
pub struct Station {
    pub q: Queues,
    pub inbox: Inbox,
}

impl Station {
    pub fn new() -> Self {
        Self { q: Queues { mgmt: TxQueue::new(0), data: TxQueue::new(TXQ_STRIDE) }, inbox: Inbox::new() }
    }
}

/// Boot the rig to ALIVE and run `f` with the firmware the join drives.
pub fn with_fw(r: &Rig, st: &mut Station, f: impl FnOnce(&mut Fw<'_, '_, Model, Mem, Polls>)) {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw_regions.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    let mut c = Polls { per_wait: 64, delays_us: 0 };
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x10D000, pnvm: None }).expect("alive");
    r.model.s.borrow_mut().commands.clear();
    let mut fw = Fw { dev: &mut dev, tx: r.tx.as_ref(), clock: &mut c, q: &mut st.q, inbox: &mut st.inbox };
    f(&mut fw);
}
