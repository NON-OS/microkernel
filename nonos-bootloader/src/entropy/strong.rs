// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The firmware's EFI_RNG_PROTOCOL, one of the two strong sources; the rule
//! over them is `strong_rule`.

use uefi::table::boot::BootServices;

/// 64 bytes from EFI_RNG_PROTOCOL into `h`; whether the firmware gave them.
/// Built where the feature asks for it, and always on aarch64, where the
/// CPU's generator is optional and the firmware's is often the only one.
#[cfg(any(feature = "efi-rng", target_arch = "aarch64"))]
pub fn collect_efi_rng(bs: &BootServices, h: &mut blake3::Hasher) -> bool {
    use uefi::proto::rng::Rng;
    use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};
    let Ok(handle) = bs.get_handle_for_protocol::<Rng>() else {
        return false;
    };
    let params = OpenProtocolParams { handle, agent: bs.image_handle(), controller: None };
    // SAFETY: GetProtocol opens without taking ownership; the driver keeps
    // the protocol for as long as boot services run, and it is dropped here.
    let opened = unsafe { bs.open_protocol::<Rng>(params, OpenProtocolAttributes::GetProtocol) };
    let Ok(mut rng) = opened else {
        return false;
    };
    let mut buf = [0u8; 64];
    let ok = rng.get_rng(None, &mut buf).is_ok() && !buf.iter().all(|&b| b == 0);
    if ok {
        h.update(&buf);
    }
    super::util::scrub(&mut buf);
    ok
}

#[cfg(not(any(feature = "efi-rng", target_arch = "aarch64")))]
pub fn collect_efi_rng(_bs: &BootServices, _h: &mut blake3::Hasher) -> bool {
    false
}

/// One word from EFI_RNG_PROTOCOL, hashed; None where the firmware gives
/// none or the protocol is not built in.
pub fn efi_rng_word(bs: &BootServices) -> Option<u64> {
    let mut h = blake3::Hasher::new();
    collect_efi_rng(bs, &mut h).then(|| {
        let b = h.finalize();
        u64::from_le_bytes(b.as_bytes()[..8].try_into().unwrap_or([0; 8]))
    })
}
