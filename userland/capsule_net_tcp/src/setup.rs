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

use nonos_libc::{crypto_random, mk_service_lookup};

use crate::ip_client::read_ipv4;
use crate::state::{set_ip_port, set_local_ip, TABLE};

const IP_NAME: &str = "net.ip";

#[derive(Clone, Copy, PartialEq, Eq)]
/// What setup found missing.
pub enum SetupError {
    /// No `net.ip` service to look up.
    Ip,
    /// The IP service gave no address.
    Config,
    /// Too little entropy to key the ISS.
    Entropy,
}

fn seed_iss_key() -> Result<(), SetupError> {
    let mut raw = [0u8; 16];
    if crypto_random(raw.as_mut_ptr(), raw.len()) != raw.len() as i64 {
        return Err(SetupError::Entropy);
    }
    let k0 = u64::from_le_bytes([raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7]]);
    let k1 =
        u64::from_le_bytes([raw[8], raw[9], raw[10], raw[11], raw[12], raw[13], raw[14], raw[15]]);
    TABLE.lock().seed_iss([k0, k1]);
    Ok(())
}

pub fn run() -> Result<(), SetupError> {
    seed_iss_key()?;
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(IP_NAME.as_ptr(), IP_NAME.len(), &mut port, &mut pid);
    if rc != 0 || port == 0 {
        return Err(SetupError::Ip);
    }
    let ip = read_ipv4(port).map_err(|_| SetupError::Config)?;
    if ip == [0, 0, 0, 0] {
        return Err(SetupError::Config);
    }
    set_ip_port(port);
    set_local_ip(ip);
    Ok(())
}
