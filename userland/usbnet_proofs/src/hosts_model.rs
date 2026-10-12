// NONOS Operating System (AGPL-3.0-or-later)
//! Modelled USB hosts: which have registered, and what sits on their root
//! ports.

use std::collections::HashMap;

use crate::scan::{Port, Tried};

pub const NAMES: &[&str] = &["driver.xhci0", "driver.ehci0", "driver.ohci0", "driver.uhci0"];

/// Hosts by name and port, and what sits on each host's root ports.
#[derive(Default)]
pub struct Model {
    pub up: Vec<(&'static str, u32)>,
    pub devices: HashMap<(u32, u8), &'static str>,
}

impl Model {
    pub fn lookup(&self, name: &str) -> Option<u32> {
        self.up.iter().find(|(n, _)| *n == name).map(|&(_, p)| p)
    }
    pub fn ports(&self, host: u32) -> Option<Vec<Port>> {
        let mut ids: Vec<u8> = self.devices.keys().filter(|k| k.0 == host).map(|k| k.1).collect();
        ids.sort();
        Some(ids.into_iter().map(|id| Port { id, owner: 0, changed: false }).collect())
    }
    /// A class driver that binds `kind`.
    pub fn try_port(&self, host: u32, port: u8, kind: &str) -> Tried<(u32, u8)> {
        match self.devices.get(&(host, port)) {
            Some(k) if *k == kind => Tried::Ours((host, port)),
            _ => Tried::NotOurs,
        }
    }
}
