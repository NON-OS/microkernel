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
 * What the store calls answer when the disk refuses. MkStoreRead and
 * MkStoreWrite share one table, included here by path against the real
 * device error and errno values.
 */

#[allow(dead_code)]
#[path = "../../../../src/syscall/microkernel/store_errno.rs"]
mod table;

#[cfg(test)]
mod tests {
    use super::table::store_errno;
    use crate::hardware::block_device::BlockDeviceError as B;
    use crate::syscall::microkernel::errnos::{
        ERRNO_ACCES, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_TIMEDOUT,
    };

    #[test]
    fn no_disk_is_no_device_for_a_read_and_a_write_alike() {
        assert_eq!(store_errno(B::Dead), ERRNO_NODEV);
    }

    #[test]
    fn every_refusal_has_its_own_answer() {
        assert_eq!(store_errno(B::AccessDenied), ERRNO_ACCES);
        assert_eq!(store_errno(B::NoCallerPid), ERRNO_ACCES);
        for e in [B::Stale, B::TransportFailure, B::NotReady] {
            assert_eq!(store_errno(e), ERRNO_TIMEDOUT, "{e:?}");
        }
        for e in [B::OutOfRange, B::InvalidArgument] {
            assert_eq!(store_errno(e), ERRNO_INVAL, "{e:?}");
        }
        for e in [B::OversizedRequest, B::DeviceFailure, B::Unsupported, B::ProtocolMismatch, B::BlockSize] {
            assert_eq!(store_errno(e), ERRNO_FAULT, "{e:?}");
        }
    }
}
