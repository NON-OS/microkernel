// NONOS Operating System (AGPL-3.0-or-later)

//! Session protection: asked for before the first frame, cancelled once the
//! keys are in or the contexts come down.

use super::super::super::cmds::MAC_CONF_GROUP;
use super::super::super::region::{Clock, Region};
use super::super::super::station::ids::SESSION_PROTECTION_CMD;
use super::super::super::station::phy::{ACTION_ADD, ACTION_REMOVE};
use super::super::super::station::session::{session_protection, JOIN_SESSION_MS};
use super::super::fw::{CommandFailed, Fw};
use super::Bss;
use crate::regs::Mmio;

impl Bss {
    /// Ask for session protection.
    pub fn protect<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
    ) -> Result<(), CommandFailed> {
        fw.command(
            MAC_CONF_GROUP,
            SESSION_PROTECTION_CMD,
            &session_protection(ACTION_ADD, JOIN_SESSION_MS),
        )?;
        self.session = true;
        Ok(())
    }

    // Cancel session protection if it is running (best effort).
    pub(super) fn unprotect<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
    ) {
        if self.session {
            self.session = false;
            let _ = fw.command(
                MAC_CONF_GROUP,
                SESSION_PROTECTION_CMD,
                &session_protection(ACTION_REMOVE, 0),
            );
        }
    }
}
