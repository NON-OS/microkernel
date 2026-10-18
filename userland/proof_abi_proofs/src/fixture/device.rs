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

//! A real device proof, bound to the record's ctx under the listing's scope.

use std::sync::OnceLock;

use nonos_attest_path::Kind;
use nonos_device_attest::{commit, prove, tag, Slot, Statement, Witness};
use nonos_proof_abi::trust::DeviceRoot;
use nonos_proof_abi::{encode_device, market_scope};
use nonos_stark::field::Fp;

use super::trees::{path_of, registry, tree, DEPTH, SECRET};

pub const LISTING: [u8; 32] = [0x4C; 32];
pub const DAY: u64 = 20_372;

pub struct Proven {
    pub boot_root: [u64; 4],
    pub kernel_root: [u64; 4],
    pub registry: DeviceRoot,
    pub section: Vec<u8>,
}

/// Every test that needs the proof shares the one made for its record.
pub fn prove_bound(ctx: [u64; 4]) -> &'static Proven {
    static ONE: OnceLock<([u64; 4], Proven)> = OnceLock::new();
    let (bound, p) = ONE.get_or_init(|| (ctx, make(ctx, market_scope(&LISTING, DAY))));
    assert_eq!(*bound, ctx, "one bound record per test run");
    p
}

/// A proof for the fixture device bound to `ctx` under `scope`.
pub(super) fn make(ctx: [u64; 4], scope: [u64; 2]) -> Proven {
    let (boot_root, boot, bd) = tree(Kind::Bootloader, b"bootloader context");
    let (kernel_root, kernel, kd) = tree(Kind::Kernel, b"kernel context");
    let s = SECRET.map(Fp::from_u64);
    let (device_root, device) = registry(commit(&s), 5);
    let e = scope.map(Fp::from_u64);
    let st = Statement {
        boot_root,
        kernel_root,
        device_root,
        device_depth: DEPTH,
        scope: e,
        context: ctx.map(Fp::from_u64),
        tag: tag(&s, &e),
    };
    let w = Witness {
        secret: s,
        bootloader: Slot { digest: bd, path: path_of(&boot, 0) },
        kernel: Slot { digest: kd, path: path_of(&kernel, 0) },
        device,
    };
    let proof = prove(&st, &w, &[0x3D; 64]).expect("a device proof");
    let words = |w: &[Fp; 4]| w.map(|f| f.to_u64());
    let registry =
        DeviceRoot { root: words(&device_root), depth: DEPTH, from_day: DAY - 30, life_days: 365 };
    let section = encode_device(&st, &proof).expect("a device section");
    Proven { boot_root: words(&boot_root), kernel_root: words(&kernel_root), registry, section }
}
