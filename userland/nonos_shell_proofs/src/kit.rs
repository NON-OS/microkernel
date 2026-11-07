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

//! A `ShellOut` that keeps every line, and a `Host` whose answers a proof
//! sets.

use nonos_shell::host::{Attest, Echo, Host, Mem, Proc, Resolve, Service};
use nonos_shell::out::ShellOut;
use std::collections::HashMap;

#[derive(Default)]
pub struct Lines(pub Vec<Vec<u8>>);

impl ShellOut for Lines {
    fn writeln(&mut self, line: &[u8]) {
        self.0.push(line.to_vec());
    }
}

impl Lines {
    /// The lines as text, for readable assertions.
    pub fn text(&self) -> Vec<String> {
        self.0.iter().map(|l| String::from_utf8_lossy(l).into_owned()).collect()
    }
}

#[derive(Default)]
pub struct Script {
    /// The byte `random` fills with; None refuses.
    pub random: Option<u8>,
    pub wall_ms: Option<u64>,
    pub uptime_ms: Option<u64>,
    pub mem: Option<Mem>,
    pub procs: Option<Vec<Proc>>,
    pub files: HashMap<Vec<u8>, Vec<u8>>,
    pub attest: Option<Attest>,
    /// The registry, by name; None means the host cannot read it.
    pub services: Option<HashMap<Vec<u8>, (u32, u32)>>,
    /// Every name `service` asked the host for, in order.
    pub asked: Vec<Vec<u8>>,
    /// What a lookup answers, and every name asked.
    pub resolve: Option<Resolve>,
    pub resolved: Vec<Vec<u8>>,
    /// What each echo answers, in turn, and every echo asked.
    pub echoes: Vec<Echo>,
    pub echoed: Vec<([u8; 4], u16)>,
}

impl Host for Script {
    fn random(&mut self, buf: &mut [u8]) -> bool {
        self.random.map(|b| buf.fill(b)).is_some()
    }
    fn wall_ms(&mut self) -> Option<u64> {
        self.wall_ms
    }
    fn uptime_ms(&mut self) -> Option<u64> {
        self.uptime_ms
    }
    fn mem(&mut self) -> Option<Mem> {
        self.mem
    }
    fn procs(&mut self) -> Option<Vec<Proc>> {
        self.procs.clone()
    }
    fn read_file(&mut self, path: &[u8], max: usize) -> Option<Vec<u8>> {
        self.files.get(path).filter(|f| f.len() <= max).cloned()
    }
    fn attest(&mut self) -> Option<Attest> {
        self.attest
    }
    fn service(&mut self, name: &[u8]) -> Service {
        self.asked.push(name.to_vec());
        match self.services.as_ref().map(|s| s.get(name)) {
            None => Service::Unanswered,
            Some(None) => Service::Absent,
            Some(Some(&(port, pid))) => Service::Found { port, pid },
        }
    }
    fn resolve(&mut self, host: &[u8]) -> Resolve {
        self.resolved.push(host.to_vec());
        self.resolve.unwrap_or(Resolve::Unavailable)
    }
    fn echo(&mut self, dst: [u8; 4], seq: u16) -> Echo {
        self.echoed.push((dst, seq));
        if self.echoes.is_empty() {
            return Echo::Unavailable;
        }
        self.echoes.remove(0)
    }
}

/// Words of a command line, split on spaces.
pub fn argv(line: &str) -> Vec<&[u8]> {
    line.split(' ').map(str::as_bytes).collect()
}
