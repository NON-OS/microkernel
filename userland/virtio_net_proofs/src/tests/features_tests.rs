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

//! What the modern negotiation takes, through the shared transport's accept:
//! ACCESS_PLATFORM whenever the device offers it, so a device behind an
//! IOMMU never bypasses it (plan/50 C3); never the device's MAC (C4).

use nonos_virtio::common::accept;
use nonos_virtio::features::{VIRTIO_F_ACCESS_PLATFORM, VIRTIO_F_VERSION_1};

use super::common::Common;
use crate::constants::{MODERN_WANTED, VIRTIO_NET_F_MAC, VIRTIO_NET_F_STATUS};

#[test]
fn access_platform_is_taken_whenever_offered_and_the_mac_never() {
    let (mac, status) = (1u64 << VIRTIO_NET_F_MAC, 1u64 << VIRTIO_NET_F_STATUS);
    for platform in [VIRTIO_F_ACCESS_PLATFORM, 0] {
        let c = Common::offering(VIRTIO_F_VERSION_1 | platform | mac | status);
        let want = VIRTIO_F_VERSION_1 | platform | status;
        assert_eq!(accept(&c, MODERN_WANTED), Ok(want));
        assert_eq!(c.taken.get(), want, "what the device was told");
    }
}
