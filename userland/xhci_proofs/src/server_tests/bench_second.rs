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

//! The bench with a second controller behind the multiplexer: its own
//! window and interrupter, its root ports numbered after the first's, and
//! nothing attached.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use super::bench::Bench;
use super::bench_driver::driver;
use crate::conformance::model::controller;
use crate::dma::DmaPool;
use crate::rings::event::EventRing;
use crate::server::{context::Context, mux::Mux};

impl Bench {
    /// Put the second controller behind the multiplexer; its window and
    /// interrupter registers, which must outlive it.
    pub fn second_controller(&mut self) -> (Arc<FakeBar>, FakeBar) {
        let (second, intr) = (controller(0), FakeBar::new(0x40));
        let pool = DmaPool::new(9, 1);
        let ring = EventRing::new(&pool).expect("event ring");
        let first = self.mux.ctxs.pop().expect("the bench's controller");
        let other = Context::new(driver(&second, pool, intr.base(), ring));
        self.mux = Mux::new(vec![first, other]);
        (second, intr)
    }
}
