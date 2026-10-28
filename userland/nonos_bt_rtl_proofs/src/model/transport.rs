// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The model as the host core's transport: each command answered at once,
//! one Command Complete each.

use nonos_bt_hci::{HciError, Transport};

use super::Rtl;

impl Transport for Rtl {
    fn send_command(&mut self, packet: &[u8]) -> Result<(), HciError> {
        let e = self.answer(packet);
        self.events.push_back(e);
        Ok(())
    }
    fn send_acl(&mut self, _: &[u8]) -> Result<(), HciError> {
        panic!("no ACL data before bring-up")
    }
    fn poll_event(&mut self) -> Result<Option<Vec<u8>>, HciError> {
        Ok(self.events.pop_front())
    }
    fn poll_acl(&mut self) -> Result<Option<Vec<u8>>, HciError> {
        Ok(None)
    }
    fn pause(&mut self) {}
}
