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

//! Events from the controller while a link is up. Disconnection Complete
//! for the link ends it (Vol 4, Part E, 7.7.5), and a link this host never
//! asked for is dealt with in `stray`, and its encryption lost in `sealed`.
//! Hardware Error never comes here:
//! the host stops on it, since the controller must be brought up again
//! (7.7.16). Everything else needs nothing from this host.

use super::end::End;
use super::env::Env;
use super::sealed::KEY_REFRESH_COMPLETE;
use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::hci::event::parse_event;
use nonos_bt_wire::hci::event_code::{DISCONNECTION_COMPLETE, ENCRYPTION_CHANGE, LE_META};
use nonos_bt_wire::hci::DisconnectionComplete;

impl Session {
    pub(super) fn on_event<T: Transport>(
        &mut self,
        host: &mut Host<T>,
        event: &[u8],
    ) -> Result<(), HciError> {
        let e = parse_event(event)?;
        match e.code {
            DISCONNECTION_COMPLETE => {
                let d = DisconnectionComplete::parse(e.params)?;
                if d.status == 0 && d.handle == self.handle {
                    self.end = Some(End::Lost(d.reason));
                }
            }
            LE_META => self.on_le(host, e.params)?,
            ENCRYPTION_CHANGE | KEY_REFRESH_COMPLETE => self.on_key_event(e.code, e.params),
            _ => {}
        }
        Ok(())
    }

    /// Every key and button still down goes up, however the link ended: a
    /// device gone mid-press sends no release of its own.
    pub(super) fn release<E: Env>(&mut self, env: &mut E) {
        for input in self.hid.as_mut().map(|h| h.disconnected()).unwrap_or_default() {
            self.post(input, env);
        }
    }
}
