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

//! The driver's loop: answer the audio server, feed the device while the
//! stream runs, and look for a device while there is none.

use alloc::vec;

use nonos_libc::Deadline;
use nonos_usbnet::scan::{Bound, Scanner};
use nonos_usbnet::xhci::disable_slot;
use nonos_usbnet::{run::own_pid, say::say};

use super::feed::feed;
use super::loss::Loss;
use super::one::serve_one;
use super::play::{Device, Play};
use super::reply::REPLY_MAX;
use crate::bind::{bind, Speaker};
use crate::wire::{HDR_LEN, MAX_PCM_CHUNK};

const TAG: &[u8] = b"usb-audio";
const SEARCH_POLL_MS: u64 = 1_000;
/// Requests served between two batches, so a burst cannot starve the device.
const DRAIN_MAX: usize = 8;

pub fn run() -> ! {
    // A batch is at most 4032 bytes (`uac::timing`), so one PCM chunk holds it.
    let (mut rx, mut batch) = (vec![0u8; HDR_LEN + MAX_PCM_CHUNK], vec![0u8; MAX_PCM_CHUNK]);
    let mut tx = [0u8; REPLY_MAX];
    let (mut play, mut scanner, me) = (Play::new(), Scanner::new(), own_pid("driver.usb_audio0"));
    let (mut bound, mut loss) = (None::<Bound<Speaker>>, Loss::default());
    let mut next_pass = Deadline::after_ms(0);
    say(TAG, &[b"started, looking for a device"]);
    loop {
        let mut wait = if play.running { 0 } else { SEARCH_POLL_MS };
        for _ in 0..DRAIN_MAX {
            if !serve_one(&mut play, &mut rx, &mut tx, wait, me) {
                break;
            }
            wait = 0;
        }
        let mut lost = false;
        if let (true, Some(b)) = (play.running, bound.as_ref()) {
            lost = loss.note(feed(&mut play, b, &mut batch).is_ok());
        }
        if lost {
            if let Some(b) = bound.take() {
                say(TAG, &[b"device stopped answering, given back"]);
                disable_slot(b.xhci, b.slot);
                scanner.lost(b.port);
            }
            loss.clear();
            play.lost();
        }
        if bound.is_none() && next_pass.expired() {
            next_pass = Deadline::after_ms(SEARCH_POLL_MS);
            bound = scanner.pass(TAG, bind);
            play.device = bound.as_ref().map(|b| Device { ids: b.nic.ids, outputs: b.nic.outputs });
        }
    }
}
