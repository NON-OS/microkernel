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

//! The host's trusted role table and revocation list, as the server reads them.

/// The host's trusted role table and revocation list. `with` hands both
/// to `f` as they stand at that moment, so a host that installs a newer
/// list has every later check read it. An error means the host can no
/// longer vouch for them; the server then refuses every login and
/// escalation and stops every session, as it would on a stale list.
pub trait Roles {
    fn with<T>(
        &self,
        f: impl FnOnce(&nsh_roles::Policy<'_>, &nsh_roles::Revocations<'_>) -> T,
    ) -> Result<T, nsh_roles::PolicyError>;
}

impl<R: Roles + ?Sized> Roles for &R {
    fn with<T>(
        &self,
        f: impl FnOnce(&nsh_roles::Policy<'_>, &nsh_roles::Revocations<'_>) -> T,
    ) -> Result<T, nsh_roles::PolicyError> {
        (**self).with(f)
    }
}

impl<R: Roles + ?Sized> Roles for alloc::rc::Rc<R> {
    fn with<T>(
        &self,
        f: impl FnOnce(&nsh_roles::Policy<'_>, &nsh_roles::Revocations<'_>) -> T,
    ) -> Result<T, nsh_roles::PolicyError> {
        (**self).with(f)
    }
}
