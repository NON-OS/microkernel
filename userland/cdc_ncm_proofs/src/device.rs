// NONOS Operating System (AGPL-3.0-or-later)
//! An NCM device as a scripted bus: its descriptors, its NTB parameters
//! for GET_NTB_PARAMETERS, its current size for GET_MAX_DATAGRAM_SIZE, and
//! every other control request accepted, or refused when `refuse` names it.

use nonos_usbnet::found::{fetch, Found};
use nonos_usbnet::mock::{descriptors, Call, MockBus};

use crate::spec::{config_ncm, device, mac_string, ntb_params};

#[derive(Clone)]
pub struct Dev {
    pub vendor: u16,
    pub configs: Vec<Vec<u8>>,
    pub params: Vec<u8>,
    pub max_dgram: u16,
    pub refuse: Option<(u8, i32)>,
}

impl Dev {
    /// One NCM configuration with `caps`; 16-bit NTBs only, 16 KiB blocks
    /// each way, the OUT side's divisor 4, remainder 0, alignment 4.
    pub fn ncm(caps: u8) -> Self {
        let params = ntb_params(0x0001, 16384, 16384, (4, 0, 4));
        Self {
            vendor: 0x0001,
            configs: vec![config_ncm(1, caps)],
            params,
            max_dgram: 1514,
            refuse: None,
        }
    }

    pub fn bus(&self) -> MockBus {
        let d = self.clone();
        let dev = device(d.vendor, d.configs.len() as u8);
        MockBus::new(Box::new(move |call| {
            let configs: Vec<&[u8]> = d.configs.iter().map(|c| c.as_slice()).collect();
            if let Some(bytes) = descriptors(call, &dev, &configs, &[(4, &mac_string())]) {
                return Ok(bytes);
            }
            match (call, d.refuse) {
                (Call::In(s, _) | Call::Out(s, _), Some((r, e))) if s.request == r => Err(e),
                (Call::In(s, _), _) if s.request == 0x80 => Ok(d.params.clone()),
                (Call::In(s, _), _) if s.request == 0x87 => Ok(d.max_dgram.to_le_bytes().to_vec()),
                _ => Ok(vec![]),
            }
        }))
    }

    /// What the search reads from the device before asking the driver.
    pub fn found(&self) -> Found {
        fetch(&mut self.bus(), 1).expect("the scripted device answers")
    }
}

/// The calls a bind made, past the descriptor reads.
pub fn calls(bus: &MockBus) -> Vec<Call> {
    bus.0.borrow().calls.clone()
}
