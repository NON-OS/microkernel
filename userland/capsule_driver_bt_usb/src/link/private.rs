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

//! LE privacy for this host: a resolvable private address renewed every
//! TGAP(private_addr_int) (Vol 3, Part C, Appendix A), given to the
//! controller only between scans and connections (Vol 4, Part E, 7.8.4).
//! Peers learn this host's IRK and public identity in pairing (Vol 3,
//! Part H, 3.6.4 and 3.6.5) and resolve each new address with it.

use super::env::Env;
use super::us::Us;
use nonos_bt_bond::{LocalIrk, Rotation};
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_smp::pairing::Identity;
use nonos_bt_wire::addr::BdAddr;

/// Draws refused by Vol 6, Part B, 1.3.2.2 come two in 2^22; a source that
/// gives this many in a row is broken, and the old address is kept.
const DRAWS: usize = 8;

pub struct Private {
    rotation: Rotation,
    identity: Identity,
}

impl Private {
    /// `public` is the controller's own address, given as the identity.
    pub fn new(irk: LocalIrk, public: BdAddr) -> Self {
        let identity = Identity { irk: *irk.key(), addr_type: 0, addr: public.0 };
        Self { rotation: Rotation::new(irk), identity }
    }

    /// The radio is coming back on: make the next `refresh` draw a fresh
    /// address, so it never reappears under the one it went off with (E-72).
    pub fn resume(&mut self) {
        self.rotation.expire();
    }

    /// The address to scan and connect as next, renewed first if its time
    /// is up. Call only while neither scanning nor connecting.
    pub fn refresh<T: Transport, E: Env>(
        &mut self,
        host: &mut Host<T>,
        env: &mut E,
    ) -> Result<Us, HciError> {
        let now = env.now_ms();
        if self.rotation.due(now) {
            for _ in 0..DRAWS {
                let mut draw = [0u8; 3];
                env.fill(&mut draw);
                if let Some(addr) = self.rotation.renew(draw, now) {
                    host.use_random_address(addr)?;
                    break;
                }
            }
        }
        let current = self.rotation.current();
        let addr = current.unwrap_or(BdAddr(self.identity.addr));
        let identity = current.map(|_| self.identity);
        Ok(Us { kind: host.own_address(), addr, identity })
    }
}
