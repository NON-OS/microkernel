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

use crate::arch::interrupt_controller::{broadcast_ipi, Ipi};

/// Stop every other cpu for a fatal halt.
///
/// On the multi-core image this is an NMI: the panic vector waits for its
/// target to unmask interrupts, and a cpu spinning masked on a lock the
/// halting cpu holds never does, so "halt the machine" left it running. The
/// vector remains the fallback when no NMI could be sent, and is what the
/// single-cpu image sends, where there is nobody to reach either way.
pub fn send_panic_ipi() {
    if cfg!(feature = "nonos-smp") && super::nmi::halt_others() {
        return;
    }
    let _ = broadcast_ipi(Ipi::Panic);
}
