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

//! Asks the kernel for this machine's key each time the Security page opens,
//! so its row says what the TPM answered then. The 32 bytes are never kept:
//! they are wiped as soon as the call returns.

use nonos_libc::{machine_key, MACHINE_KEY_NO_TPM, MACHINE_KEY_WRONG_STATE};
use nonos_wifi_client::wipe;

use super::machine_key::{classify, MachineKey, NO_TPM, WRONG_STATE};

const _: () = assert!(NO_TPM == MACHINE_KEY_NO_TPM && WRONG_STATE == MACHINE_KEY_WRONG_STATE);

/// A label of its own, so the key it gives is one that seals nothing.
const LABEL: &[u8] = b"settings/security-probe";

pub fn probe_machine_key() -> MachineKey {
    classify(machine_key(LABEL).map(|mut key| wipe(&mut key)))
}
