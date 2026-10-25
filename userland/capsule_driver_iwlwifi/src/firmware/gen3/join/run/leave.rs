// NONOS Operating System (AGPL-3.0-or-later)

//! Leaving a joined network, and forgetting one the firmware lost.

use super::super::super::region::{Clock, Region};
use super::super::fw::{Fw, Queue};
use super::ends::Joined;
use super::LEAVE_MS;
use crate::regs::Mmio;

/// Leave: tell the access point, close the port, remove the keys and take the
/// contexts down.
pub fn leave<M: Mmio, R: Region + ?Sized, C: Clock>(fw: &mut Fw<'_, '_, M, R, C>, j: &mut Joined) {
    if let Some(deauth) = j.link.deauth() {
        if fw.send(Queue::Mgmt, &deauth, j.link.mgmt_rate) {
            fw.settle(LEAVE_MS);
        }
    }
    forget(fw, j);
}

/// Close the port, remove the keys and take the contexts down, telling the
/// access point nothing (it already ended the association).
pub fn forget<M: Mmio, R: Region + ?Sized, C: Clock>(fw: &mut Fw<'_, '_, M, R, C>, j: &mut Joined) {
    j.link.station.deassociate();
    j.keys.remove(fw);
    j.bss.teardown(fw);
}
