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

//! A PCIe tree of root ports and switches, buses numbered depth first as
//! enumeration numbers them, its shape drawn by `pick(n)`, a number below n.

use super::acs_span::Port;
type Pick<'a> = &'a mut dyn FnMut(u64) -> u64;

/// An endpoint: its bus, and the (switch, port) pairs above it, outermost first.
pub struct Device {
    pub bus: u8,
    pub above: Vec<(usize, usize)>,
}

#[derive(Default)]
pub struct Tree {
    pub ports: Vec<Port>,
    pub port_of: Vec<(usize, usize)>,
    pub devices: Vec<Device>,
    switches: usize,
    buses: u8,
}

impl Tree {
    pub fn build(pick: Pick) -> Tree {
        let mut t = Tree { buses: 1, ..Tree::default() };
        for _ in 0..1 + pick(4) {
            t.child(pick, &mut vec![], 0);
        }
        t
    }

    // Below a port, on its next bus: an endpoint, or a switch of 1 to 4 ports.
    fn child(&mut self, pick: Pick, above: &mut Vec<(usize, usize)>, depth: u32) {
        let bus = self.buses;
        self.buses += 1;
        if depth >= 3 || pick(2) == 0 {
            self.devices.push(Device { bus, above: above.clone() });
            return;
        }
        let (id, inside) = (self.switches, self.buses);
        (self.switches, self.buses) = (id + 1, inside + 1);
        for k in 0..1 + pick(4) as usize {
            let first = self.buses;
            above.push((id, k));
            self.child(pick, above, depth + 1);
            above.pop();
            let below = Some((first, self.buses - 1));
            self.ports.push(Port { bus: inside, below, closed: pick(2) == 0 });
            self.port_of.push((id, k));
        }
    }

    /// Whether `x`'s request reaches `y` without going up to the root: at the
    /// lowest switch above both, `x`'s port routes it across unless closed.
    pub fn reaches(&self, x: &Device, y: &Device) -> bool {
        let mut shared = x.above.iter().zip(&y.above).take_while(|(a, b)| a.0 == b.0);
        let Some((at, _)) = shared.find(|(a, b)| a.1 != b.1) else { return false };
        self.port_of.iter().position(|p| p == at).is_some_and(|i| !self.ports[i].closed)
    }
}
