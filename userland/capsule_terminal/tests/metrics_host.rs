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
//
// Host-side harness for the telemetry rail's arithmetic. Compiled with the
// host toolchain:
//   rustc --edition 2021 --test tests/metrics_host.rs -o /tmp/metrics_host && /tmp/metrics_host
//
// Only the allocation-free, syscall-free half of `src/rail` is pulled in:
// `sample.rs` reaches for `nonos_libc` and cannot link on the host, while the
// derivations and the sparkline window are pure and are what this asserts.

#[path = "metrics_host/cpu.rs"]
mod cpu;
#[path = "metrics_host/mounts.rs"]
mod mounts;
#[path = "metrics_host/ring.rs"]
mod ring;
#[path = "metrics_host/sample.rs"]
mod sample;

use mounts::rail;
