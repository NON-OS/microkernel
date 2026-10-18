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

//! One controller behind the server, the model running against it, and a
//! device on root port 1. Server tests run one at a time: the server keeps
//! its reply routing in statics, as a single-threaded capsule may, and each
//! starts as the kernel's sender, whatever the one before left.

use std::sync::{Arc, Mutex, MutexGuard};

use nonos_devmodel::{run, FakeBar, LiveDevice};

use super::bench_driver::{connected, driver};
use super::hc::{Hc, SLOTS};
use super::hc_device::{controller as model, IDLE};
use crate::conformance::model::{controller, CAP_LEN, DOORBELL};
use crate::conformance::port_device::attached_at;
use crate::constants::{USBSTS, USBSTS_HCH};
use crate::controller::program_event_ring;
use crate::dma::DmaPool;
use crate::event_ring::producer::Producer;
use crate::rings::event::EventRing;
use crate::server::{context::Context, mux::Mux, reply::set_sender};

pub struct Bench {
    pub mux: Mux,
    pub hc: Arc<Mutex<Hc>>,
    pub bar: Arc<FakeBar>,
    _devices: [LiveDevice; 2],
    _one_at_a_time: MutexGuard<'static, ()>,
}

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

impl Bench {
    /// A device at protocol speed id `speed` on root port 1.
    pub fn new(speed: u32) -> Self {
        let serial = ONE_AT_A_TIME.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        set_sender(0);
        let bar = controller(0);
        bar.present32(CAP_LEN + USBSTS as usize, USBSTS_HCH);
        for slot in 0..=SLOTS as usize {
            bar.present32(DOORBELL + 4 * slot, IDLE);
        }
        let pool = DmaPool::new(9, 1);
        let event_ring = EventRing::new(&pool).expect("event ring");
        let mut producer = Producer::new();
        program_event_ring(producer.regs.base(), &event_ring);
        producer.start();
        let driver = driver(&bar, pool, producer.regs.base(), event_ring);
        let hc = Arc::new(Mutex::new(Hc::new(producer)));
        let devices = [run(&bar, model(hc.clone())), run(&bar, attached_at(true, speed))];
        connected(&bar);
        let mux = Mux::new(vec![Context::new(driver)]);
        Self { mux, hc, bar, _devices: devices, _one_at_a_time: serial }
    }
}
