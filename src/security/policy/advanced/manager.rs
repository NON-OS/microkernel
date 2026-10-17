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

//! The advanced security manager.

use super::*;
use core::sync::atomic::{AtomicU64, Ordering};

pub struct AdvancedSecurityManager {
    pub config: SecurityConfig,
    pub stack_canary: StackCanary,
    pub cfi: CFI,
    pub aslr: ASLR,
    pub violations: AtomicU64,
}

impl AdvancedSecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            stack_canary: StackCanary::new(),
            cfi: CFI::new(),
            aslr: ASLR::new(28),
            config,
            violations: AtomicU64::new(0),
        }
    }
    pub fn init(&self) {
        if self.config.enable_stack_canaries {
            self.stack_canary.rotate();
        }
        if self.config.enable_cfi {
            self.cfi.enable();
        }
        if self.config.enable_aslr {
            self.aslr.enabled.store(true, Ordering::Release);
        }
    }
    pub fn report_violation(&self, msg: &str) {
        self.violations.fetch_add(1, Ordering::Release);
        crate::log::security_log!("SECURITY VIOLATION: {}", msg);
    }
    pub fn stats(&self) -> SecurityStats {
        SecurityStats {
            violations: self.violations.load(Ordering::Relaxed),
            canary_rotations: self.stack_canary.rotations.load(Ordering::Relaxed),
            cfi_violations: self.cfi.violations.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug)]
pub struct SecurityStats {
    pub violations: u64,
    pub canary_rotations: u64,
    pub cfi_violations: u64,
}
