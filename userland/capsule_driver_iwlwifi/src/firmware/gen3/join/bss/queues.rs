// NONOS Operating System (AGPL-3.0-or-later)

//! The access point's management and data queues, each ring starting at the
//! write pointer the firmware's reply names.

use super::super::super::region::{Clock, Region};
use super::super::super::station::ids::{DATA_PATH_GROUP, SCD_QUEUE_CONFIG_CMD};
use super::super::super::station::queue::{add_queue, parse_queue_reply, DATA_TID, MGMT_TID};
use super::super::super::station::sta::AP_STA_MASK;
use super::super::super::txq::TXQ_ENTRIES;
use super::super::fw::Fw;
use super::{Bss, SetupError, Stage};
use crate::regs::Mmio;

impl Bss {
    // Add the management queue, then the data queue.
    pub(super) fn add_queues<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
    ) -> Result<(), SetupError> {
        for (tid, stage) in [(MGMT_TID, Stage::MgmtQueue), (DATA_TID, Stage::DataQueue)] {
            let q = if tid == MGMT_TID { &mut fw.q.mgmt } else { &mut fw.q.data };
            if !q.clear(fw.tx) {
                return Err(SetupError::Region);
            }
            let cmd = add_queue(AP_STA_MASK, tid, TXQ_ENTRIES, q.bc(fw.tx), q.tfds(fw.tx));
            let reply = fw.command(DATA_PATH_GROUP, SCD_QUEUE_CONFIG_CMD, &cmd)?;
            let given = parse_queue_reply(&reply).ok_or(SetupError::BadQueueReply)?;
            let q = if tid == MGMT_TID { &mut fw.q.mgmt } else { &mut fw.q.data };
            q.start(given.queue, given.write_ptr);
            self.stage = stage;
        }
        Ok(())
    }
}
