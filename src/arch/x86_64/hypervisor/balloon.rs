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

/*
 * A memory balloon the host offers and this kernel leaves alone. Ballooning
 * hands guest memory back to the host on request, and a host that can size
 * a guest's memory and read its free page reports learns how the guest
 * runs; whether NONOS ever lends memory that way is the owner's decision,
 * so the safe default is off: no driver binds the device, VirtualBox's
 * balloon events are filtered out by its guest driver, and VMBus offers no
 * dynamic memory channel here. The boot line says so when a balloon exists.
 */

const VIRTIO_VENDOR: u16 = 0x1AF4;
/// virtio-balloon: transitional and modern device ids.
const VIRTIO_BALLOON_LEGACY: u16 = 0x1002;
const VIRTIO_BALLOON: u16 = 0x1045;

/// Whether the device table holds a balloon device.
pub(super) fn balloon_offered(mut ids: impl Iterator<Item = (u16, u16)>) -> bool {
    ids.any(|(v, d)| v == VIRTIO_VENDOR && (d == VIRTIO_BALLOON_LEGACY || d == VIRTIO_BALLOON))
}
