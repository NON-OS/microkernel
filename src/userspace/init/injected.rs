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

//! What a proof image injects into init. Each is a no-op unless its feature
//! is on, and none can be built into an image the seal would release.
//!
//! `nonos-panic-proof` panics once the spawn plan has run, with every CPU up
//! and the capsules started, so a boot shows the panic path end to end: the
//! other CPUs stopped and confirmed, the heap wiped, the machine powered off
//! (`boot/panic/handler.rs`, `security/zerostate/panic_wipe.rs`). QEMU
//! exiting after "[ZEROSTATE] panic: cpus halted, wiping, powering off" is
//! the evidence. `nonos-fault-proof` raises a kernel #UD there instead, so
//! a boot shows a CPU exception's kernel branch ending the same way, after
//! "[ZEROSTATE] fatal kernel fault".

#[cfg(all(
    feature = "nonos-panic-proof",
    any(feature = "nonos-production", feature = "nonos-release")
))]
compile_error!("nonos-panic-proof panics the kernel on purpose; it never ships in a release");

#[cfg(all(
    feature = "nonos-fault-proof",
    any(feature = "nonos-production", feature = "nonos-release")
))]
compile_error!("nonos-fault-proof faults the kernel on purpose; it never ships in a release");

/// The booted spawn gate's broken capsules (`attest_refusal`).
pub(super) fn attest_refusal() {
    #[cfg(feature = "nonos-attest-refusal-smoketest")]
    crate::userspace::attest_refusal::run();
}

pub(super) fn panic() {
    #[cfg(feature = "nonos-panic-proof")]
    panic!("nonos-panic-proof: injected after the spawn plan");
}

pub(super) fn fault() {
    #[cfg(all(feature = "nonos-fault-proof", target_arch = "x86_64"))]
    {
        crate::sys::serial::println(b"[INIT] nonos-fault-proof: ud2 in the kernel");
        // SAFETY: none needed for the machine's sake: ud2 raises #UD in
        // kernel mode, whose handler ends the machine through the panic
        // path. That is the point of this proof image.
        unsafe { core::arch::asm!("ud2") }
    }
}
