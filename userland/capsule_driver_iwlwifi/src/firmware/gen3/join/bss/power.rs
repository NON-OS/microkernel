// NONOS Operating System (AGPL-3.0-or-later)

//! The station's power command, sent as Linux's `iwl_mvm_power_update_mac`
//! is: on association with power management off, once the keys are in with
//! it on (the handshake runs awake), and off again as the contexts come down.

use super::super::super::cmds::LONG_GROUP;
use super::super::super::region::{Clock, Region};
use super::super::super::station::ids::MAC_PM_POWER_TABLE;
use super::super::super::station::power::{keep_alive_s, mac_power};
use super::super::fw::{CommandFailed, Fw};
use super::Bss;
use crate::regs::Mmio;

impl Bss {
    /// Send the station's power command, power management on when `managed`.
    pub(super) fn power<M: Mmio, R: Region + ?Sized, C: Clock>(
        &self,
        fw: &mut Fw<'_, '_, M, R, C>,
        managed: bool,
    ) -> Result<(), CommandFailed> {
        let t = &self.target;
        let keep_alive = keep_alive_s(t.dtim_period, t.beacon_int);
        fw.command(LONG_GROUP, MAC_PM_POWER_TABLE, &mac_power(keep_alive, managed))?;
        Ok(())
    }
}
