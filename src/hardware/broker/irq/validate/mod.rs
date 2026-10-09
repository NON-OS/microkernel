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

//! Pure checks on a `MkIrqBind` request, one file per delivery kind.

mod intx;
mod msi;
mod msix;
mod msix_view;

pub(super) use intx::validate_intx_request;
pub(super) use msi::validate_msi_request;
pub(super) use msix::validate_msix_request;
pub(super) use msix_view::MsixHandleView;
