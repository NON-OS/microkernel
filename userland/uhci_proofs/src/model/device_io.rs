// NONOS Operating System (AGPL-3.0-or-later)
//! A device's packets out and its toggle checks.

use super::device::{Device, Reply};

impl Device {
    /// An OUT packet: true when its toggle is the one expected (else the
    /// data is a repeat and is dropped, and the error counted).
    pub(super) fn expect(&mut self, i: usize, toggle: bool) -> bool {
        if self.toggles[i] != toggle {
            self.toggle_errors += 1;
            return false;
        }
        self.toggles[i] = !toggle;
        true
    }

    /// An IN packet sent with the endpoint's toggle, checked against the
    /// one the host asked with.
    pub(super) fn sent(&mut self, i: usize, toggle: bool, data: Vec<u8>) -> Reply {
        if self.toggles[i] != toggle {
            self.toggle_errors += 1;
        }
        self.toggles[i] = !self.toggles[i];
        Reply::Ack(data)
    }

    /// Bulk IN: the front message in packets; one shorter than a packet
    /// ends it.
    pub(super) fn bulk(&mut self, toggle: bool, max: usize) -> Reply {
        let Some(msg) = self.bulk_in.front_mut() else { return Reply::Nak };
        let n = msg.len().min(self.bulk_mps).min(max);
        let chunk: Vec<u8> = msg.drain(..n).collect();
        if msg.is_empty() {
            self.bulk_in.pop_front();
        }
        self.sent(5, toggle, chunk)
    }

    /// Queue `bytes` as one bulk IN message.
    pub fn send(&mut self, bytes: &[u8]) {
        self.bulk_in.push_back(bytes.to_vec());
    }
}
