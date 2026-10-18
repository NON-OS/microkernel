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

//! The ambient capability set and the bits it may never hold (inherit.rs).

use crate::capabilities::Capability;

// Every process needs these to live: invoke core lifecycle syscalls
// (`MkExit`, `MkYield`), talk to capsules over IPC, and allocate
// user memory through `MkMmap`. Anything beyond this set — hardware
// authority, debug, admin, graphics — is granted per-capsule by the
// spawner. `RegisterService` and `Network`/`FileSystem`/`Crypto`/
// `Hardware` are not part of the active syscall surface today and
// are deliberately excluded from the ambient.
pub(crate) const AMBIENT_CAPS: u64 =
    Capability::CoreExec.bit() | Capability::IPC.bit() | Capability::Memory.bit();

// Bits that must never appear in `AMBIENT_CAPS` in any production
// build. Hardware authority and graphics flow through explicit
// grants only; Admin and Debug are out of the ambient set entirely.
const FORBIDDEN_AMBIENT: u64 = Capability::Admin.bit()
    | Capability::Driver.bit()
    | Capability::DeviceEnum.bit()
    | Capability::Mmio.bit()
    | Capability::Irq.bit()
    | Capability::Dma.bit()
    | Capability::Pio.bit()
    | Capability::Debug.bit()
    | Capability::GraphicsDisplayQuery.bit()
    | Capability::GraphicsSurfaceCreate.bit()
    | Capability::GraphicsSurfaceMap.bit()
    | Capability::GraphicsPresent.bit()
    | Capability::SpawnBroker.bit()
    | Capability::DeviceSecret.bit()
    | Capability::InputConsumer.bit();

const _: () = assert!(
    AMBIENT_CAPS & FORBIDDEN_AMBIENT == 0,
    "AMBIENT_CAPS holds a FORBIDDEN_AMBIENT bit: hardware, Admin, Debug, graphics, SpawnBroker, \
     DeviceSecret or InputConsumer"
);
