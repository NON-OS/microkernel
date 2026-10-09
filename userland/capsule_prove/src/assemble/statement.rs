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

//! The statement and the witness, once every part is checked. The secret is
//! taken last and only as words below p, as `MkDeviceSecret` draws them: one
//! reduced would be another secret, whose commitment nobody enrolled.

use nonos_device_attest::{commit, scope, tag, words_of, Statement, Witness};

use super::enrolled::Enrolled;
use super::error::Refusal;
use super::request::Request;
use super::slots::Slots;
use super::words::canonical;

pub fn assemble(
    req: &Request,
    device: Enrolled,
    slots: Slots,
    secret: &[u8; 32],
) -> Result<(Statement, Witness), Refusal> {
    if !canonical(secret) {
        return Err(Refusal::SecretWord);
    }
    let s = words_of(secret);
    if commit(&s) != words_of(&device.commitment) {
        return Err(Refusal::CommitmentMismatch);
    }
    let e = scope(&req.verifier, req.window).ok_or(Refusal::Scope)?;
    let statement = Statement {
        boot_root: words_of(&slots.boot_root),
        kernel_root: words_of(&slots.kernel_root),
        device_root: words_of(&req.device_root),
        device_depth: device.depth,
        scope: e,
        context: words_of(&req.nonce),
        tag: tag(&s, &e),
    };
    let witness = Witness {
        secret: s,
        bootloader: slots.bootloader,
        kernel: slots.kernel,
        device: device.path,
    };
    Ok((statement, witness))
}
