// NONOS Operating System (AGPL-3.0-or-later)
//! A device: EP0 (ep0.rs), an interrupt IN endpoint 1 with reports queued,
//! a bulk IN endpoint 2 with messages queued, and a bulk OUT endpoint 3
//! that keeps what it is sent. Every data packet's toggle is checked
//! against the one the endpoint expects; a wrong one is counted.

use std::collections::VecDeque;

use super::ep0::Ep0;
use super::pid::{PID_IN, PID_OUT};

pub enum Reply {
    Ack(Vec<u8>),
    Nak,
    Stall,
}

#[derive(Default)]
pub struct Device {
    pub low_speed: bool,
    pub addr: u8,
    pub mps0: u8,
    pub desc: [u8; 18],
    pub config: Vec<u8>,
    pub reports: VecDeque<Vec<u8>>,
    pub bulk_in: VecDeque<Vec<u8>>,
    pub bulk_mps: usize,
    pub bulk_out: Vec<u8>,
    pub stall_bulk_in: bool,
    /// Answers nothing but NAK.
    pub mute: bool,
    /// The bRequest EP0 stalls.
    pub stall_request: Option<u8>,
    pub requests: Vec<[u8; 8]>,
    pub ctrl_out: Vec<u8>,
    pub toggle_errors: u32,
    pub(super) ep0: Ep0,
    /// The toggle each endpoint's next packet carries, by `2 * ep + in`.
    pub(super) toggles: [bool; 32],
    /// A hub: its ports and what is on them (hub.rs).
    pub hub: Option<Box<super::hub::HubModel>>,
}

impl Device {
    pub fn packet(&mut self, pid: u8, ep: u8, toggle: bool, out: &[u8], max: usize) -> Reply {
        match (ep, pid) {
            _ if self.mute => Reply::Nak,
            (0, _) => super::ep0::packet(self, pid, toggle, out, max),
            (1, PID_IN) => match self.reports.pop_front() {
                Some(r) => self.sent(3, toggle, r),
                None => Reply::Nak,
            },
            (2, PID_IN) if self.stall_bulk_in => Reply::Stall,
            (2, PID_IN) => self.bulk(toggle, max),
            (3, PID_OUT) => {
                if self.expect(6, toggle) {
                    self.bulk_out.extend_from_slice(out);
                }
                Reply::Ack(vec![])
            }
            _ => Reply::Stall,
        }
    }
}
