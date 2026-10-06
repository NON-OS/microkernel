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

//! A modern device's common configuration, modelled on QEMU's
//! (virtio_pci_common_read and _write in hw/virtio/virtio-pci.c): feature
//! pages behind their select registers, a reset on a zero status write,
//! FEATURES_OK withheld when the driver took a bit it was not offered or
//! left out VERSION_1, per-queue registers behind queue_select, and
//! NO_VECTOR read back for a vector the device does not have. Every status
//! write is logged so the order of the handshake can be checked.

use std::cell::{Ref, RefCell};

use crate::common::*;
use crate::features::VIRTIO_F_VERSION_1;

#[derive(Clone, Copy)]
pub struct Queue {
    pub max: u16,
    pub size: u16,
    pub vector: u16,
    pub enable: u16,
    pub notify_off: u16,
    pub desc: u64,
    pub driver: u64,
    pub device: u64,
    /// Whether the queue was enabled when DRIVER_OK arrived.
    pub enabled_at_driver_ok: bool,
}

pub struct State {
    pub offered: u64,
    pub dfsel: u32,
    pub gfsel: u32,
    pub driver_features: u64,
    pub status: u8,
    pub status_writes: Vec<u8>,
    /// The status never reads back zero after a reset.
    pub stuck_reset: bool,
    /// FEATURES_OK is withheld whatever was taken.
    pub refuse_features: bool,
    /// queue_enable never reads back set.
    pub enable_stuck: bool,
    pub nvectors: u16,
    pub config_vector: u16,
    pub queue_sel: u16,
    pub queues: Vec<Queue>,
    pub generation: u8,
    /// The generation moves on this many of the next reads.
    pub generation_moves: u32,
}

pub struct ModelDevice(RefCell<State>);

impl ModelDevice {
    /// A device offering `offered`, with one queue per entry of `queue_max`
    /// (its maximum size; zero means the queue does not exist). Queue N
    /// reports notify offset N, as QEMU does.
    pub fn new(offered: u64, queue_max: &[u16]) -> Self {
        let queues = queue_max
            .iter()
            .enumerate()
            .map(|(i, &max)| Queue {
                max,
                size: max,
                vector: NO_VECTOR,
                enable: 0,
                notify_off: i as u16,
                desc: 0,
                driver: 0,
                device: 0,
                enabled_at_driver_ok: false,
            })
            .collect();
        Self(RefCell::new(State {
            offered,
            dfsel: 0,
            gfsel: 0,
            driver_features: 0,
            status: STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_DRIVER_OK,
            status_writes: Vec::new(),
            stuck_reset: false,
            refuse_features: false,
            enable_stuck: false,
            nvectors: 0,
            config_vector: NO_VECTOR,
            queue_sel: 0,
            queues,
            generation: 0,
            generation_moves: 0,
        }))
    }

    pub fn state(&self) -> Ref<'_, State> {
        self.0.borrow()
    }

    pub fn with(&self, f: impl FnOnce(&mut State)) {
        f(&mut self.0.borrow_mut())
    }
}

fn page(value: u64, select: u32) -> u32 {
    match select {
        0 => value as u32,
        1 => (value >> 32) as u32,
        _ => 0,
    }
}

impl State {
    fn queue(&mut self) -> Option<&mut Queue> {
        let sel = self.queue_sel as usize;
        self.queues.get_mut(sel)
    }

    fn write_status(&mut self, v: u8) {
        self.status_writes.push(v);
        if v == 0 {
            if !self.stuck_reset {
                self.reset();
            }
            return;
        }
        let mut v = v;
        let new = v & !self.status;
        if new & STATUS_FEATURES_OK != 0 {
            let taken_unoffered = self.driver_features & !self.offered != 0;
            let no_version = self.driver_features & VIRTIO_F_VERSION_1 == 0;
            if self.refuse_features || taken_unoffered || no_version {
                v &= !STATUS_FEATURES_OK;
            }
        }
        if new & STATUS_DRIVER_OK != 0 {
            for q in &mut self.queues {
                q.enabled_at_driver_ok = q.enable == 1;
            }
        }
        self.status = v;
    }

    fn reset(&mut self) {
        self.status = 0;
        self.driver_features = 0;
        self.config_vector = NO_VECTOR;
        for q in &mut self.queues {
            q.size = q.max;
            q.vector = NO_VECTOR;
            q.enable = 0;
            q.desc = 0;
            q.driver = 0;
            q.device = 0;
        }
    }

    fn set_half(target: &mut u64, high: bool, v: u32) {
        *target = if high {
            (*target & 0xFFFF_FFFF) | ((v as u64) << 32)
        } else {
            (*target & !0xFFFF_FFFF) | v as u64
        };
    }
}

impl CommonCfg for ModelDevice {
    fn r8(&self, off: usize) -> u8 {
        let mut s = self.0.borrow_mut();
        match off {
            DEVICE_STATUS => s.status,
            CONFIG_GENERATION => {
                let g = s.generation;
                if s.generation_moves > 0 {
                    s.generation_moves -= 1;
                    s.generation = s.generation.wrapping_add(1);
                }
                g
            }
            _ => 0,
        }
    }

    fn r16(&self, off: usize) -> u16 {
        let mut s = self.0.borrow_mut();
        let enable_stuck = s.enable_stuck;
        match off {
            MSIX_CONFIG => s.config_vector,
            NUM_QUEUES => s.queues.len() as u16,
            QUEUE_SELECT => s.queue_sel,
            QUEUE_SIZE => s.queue().map_or(0, |q| q.size),
            QUEUE_MSIX_VECTOR => s.queue().map_or(NO_VECTOR, |q| q.vector),
            QUEUE_ENABLE => s.queue().map_or(0, |q| if enable_stuck { 0 } else { q.enable }),
            QUEUE_NOTIFY_OFF => s.queue().map_or(0, |q| q.notify_off),
            _ => 0,
        }
    }

    fn r32(&self, off: usize) -> u32 {
        let s = self.0.borrow();
        match off {
            DEVICE_FEATURE_SELECT => s.dfsel,
            DEVICE_FEATURE => page(s.offered, s.dfsel),
            DRIVER_FEATURE_SELECT => s.gfsel,
            DRIVER_FEATURE => page(s.driver_features, s.gfsel),
            _ => 0,
        }
    }

    fn w8(&self, off: usize, value: u8) {
        if off == DEVICE_STATUS {
            self.0.borrow_mut().write_status(value);
        }
    }

    fn w16(&self, off: usize, value: u16) {
        let mut s = self.0.borrow_mut();
        let nvectors = s.nvectors;
        let take = |v: u16| if v < nvectors { v } else { NO_VECTOR };
        match off {
            MSIX_CONFIG => s.config_vector = take(value),
            QUEUE_SELECT => s.queue_sel = value,
            QUEUE_SIZE => {
                if let Some(q) = s.queue() {
                    q.size = value;
                }
            }
            QUEUE_MSIX_VECTOR => {
                if let Some(q) = s.queue() {
                    q.vector = take(value);
                }
            }
            QUEUE_ENABLE => {
                if let Some(q) = s.queue() {
                    q.enable = value;
                }
            }
            _ => {}
        }
    }

    fn w32(&self, off: usize, value: u32) {
        let mut s = self.0.borrow_mut();
        match off {
            DEVICE_FEATURE_SELECT => s.dfsel = value,
            DRIVER_FEATURE_SELECT => s.gfsel = value,
            DRIVER_FEATURE => {
                let gfsel = s.gfsel;
                if gfsel <= 1 {
                    let shift = 32 * gfsel;
                    let mask = 0xFFFF_FFFFu64 << shift;
                    s.driver_features = (s.driver_features & !mask) | ((value as u64) << shift);
                }
            }
            o if (QUEUE_DESC..QUEUE_DEVICE + 8).contains(&o) => {
                if let Some(q) = s.queue() {
                    let high = (o - QUEUE_DESC) % 8 == 4;
                    let target = match (o - QUEUE_DESC) / 8 {
                        0 => &mut q.desc,
                        1 => &mut q.driver,
                        _ => &mut q.device,
                    };
                    State::set_half(target, high, value);
                }
            }
            _ => {}
        }
    }
}
