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

use x86_64::structures::idt::InterruptStackFrame;

use super::context::{log_exception, ExceptionContext};
use super::nmi_source::{identify_nmi_source, NmiSource};

pub fn handle(frame: InterruptStackFrame) {
    /*
     * The kernel's own NMIs first: a fatal halt of the machine, or a TLB
     * shootdown round re-sent to a cpu that did not take the vector. That
     * step is NMI-safe; what follows logs, and is reached for an NMI nothing
     * in the kernel sent, or when a hardware cause is latched in port B
     * alongside ours, so a coincident parity or channel check is not lost.
     */
    if cfg!(feature = "nonos-smp")
        && crate::smp::nmi::on_nmi()
        && matches!(identify_nmi_source(), NmiSource::Unknown)
    {
        return;
    }
    let ctx = ExceptionContext::from_frame(&frame);
    log_exception("NMI", &ctx);

    let source = identify_nmi_source();
    handle_nmi_source(source, &ctx);
}

fn handle_nmi_source(source: NmiSource, _ctx: &ExceptionContext) {
    match source {
        NmiSource::MemoryParity => {
            crate::log::logger::log_critical("NMI: Memory parity error detected");
            handle_memory_error();
        }
        NmiSource::IoChannelCheck => {
            crate::log::logger::log_critical("NMI: I/O channel check error");
            handle_io_error();
        }
        NmiSource::Watchdog => {
            crate::log::logger::log_warning!("NMI: Watchdog timeout");
            handle_watchdog();
        }
        NmiSource::Unknown => {
            crate::log::logger::log_warning!("NMI: Unknown source");
        }
    }
}

fn handle_memory_error() {
    crate::log::logger::log_critical("Memory subsystem error - system may be unstable");
}

fn handle_io_error() {
    crate::log::logger::log_critical("I/O subsystem error - peripheral failure possible");
}

fn handle_watchdog() {
    crate::log::logger::log_warning!("System watchdog triggered");
}
