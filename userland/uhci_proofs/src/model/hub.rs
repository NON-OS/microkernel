// NONOS Operating System (AGPL-3.0-or-later)
//! A hub as a device: its class requests on EP0 (USB 2.0 section 11.24),
//! its downstream ports and the devices on them. A port reset ends at
//! once, its device back at address 0 and the port enabled. Host-neutral,
//! so every host's model mounts it.

use super::device::Device;

#[derive(Default)]
pub struct Down {
    pub dev: Option<Device>,
    /// The host wire's speed ids: 1 full, 2 low, 3 high.
    pub speed: u8,
    pub enabled: bool,
    pub powered: bool,
    pub c_connection: bool,
    pub c_reset: bool,
    pub resets: u32,
}

#[derive(Default)]
pub struct HubModel {
    pub high: bool,
    pub ports: Vec<Down>,
}

impl HubModel {
    pub fn new(high: bool, ports: usize) -> Self {
        Self { high, ports: (0..ports).map(|_| Down::default()).collect() }
    }

    /// `dev` at `speed` plugged into `port` (from 1).
    pub fn plug(mut self, port: usize, dev: Device, speed: u8) -> Self {
        let p = &mut self.ports[port - 1];
        (p.dev, p.speed, p.c_connection) = (Some(dev), speed, true);
        self
    }

    /// A class request's IN data, its OUT effect done; None for a request
    /// that is not the hub class's.
    pub fn request(&mut self, s: [u8; 8]) -> Option<Vec<u8>> {
        let (value, port) = (u16::from_le_bytes([s[2], s[3]]), s[4] as usize);
        let valid = (1..=self.ports.len()).contains(&port);
        let p = valid.then(|| port - 1);
        match (s[0], s[1], value, p) {
            (0xA0, 6, 0x2900, _) => Some(vec![9, 0x29, self.ports.len() as u8, 1 << 5, 0, 50, 0]),
            (0xA3, 0, _, Some(i)) => Some(self.status(i).to_vec()),
            (0x23, 3, 4, Some(i)) => {
                let d = &mut self.ports[i];
                d.resets += 1;
                if let Some(dev) = d.dev.as_mut() {
                    dev.reset();
                    (d.enabled, d.c_reset) = (true, true);
                }
                Some(vec![])
            }
            (0x23, 3, 8, Some(i)) => Some(vec![]).inspect(|_| self.ports[i].powered = true),
            (0x23, 1, 16, Some(i)) => Some(vec![]).inspect(|_| self.ports[i].c_connection = false),
            (0x23, 1, 20, Some(i)) => Some(vec![]).inspect(|_| self.ports[i].c_reset = false),
            _ => None,
        }
    }

    fn status(&self, i: usize) -> [u8; 4] {
        let d = &self.ports[i];
        let mut st = d.dev.is_some() as u16 | (d.enabled as u16) << 1 | (d.powered as u16) << 8;
        st |= ((d.speed == 2) as u16) << 9 | ((d.speed == 3) as u16) << 10;
        let ch = d.c_connection as u16 | (d.c_reset as u16) << 4;
        let ([a, b], [c, e]) = (st.to_le_bytes(), ch.to_le_bytes());
        [a, b, c, e]
    }
}
