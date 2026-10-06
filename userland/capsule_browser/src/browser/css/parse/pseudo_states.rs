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

use crate::browser::css::selector::{FormState as F, Pseudo as P, UserState as U};

/* The form-control and user-interaction pseudo-classes, by lower-cased
 * name. */
pub(super) fn state(name: &str) -> Option<P> {
    Some(match name {
        "enabled" => P::Form(F::Enabled),
        "disabled" => P::Form(F::Disabled),
        "checked" => P::Form(F::Checked),
        "default" => P::Form(F::Default),
        "indeterminate" => P::Form(F::Indeterminate),
        "required" => P::Form(F::Required),
        "optional" => P::Form(F::Optional),
        "read-only" => P::Form(F::ReadOnly),
        "read-write" => P::Form(F::ReadWrite),
        "placeholder-shown" => P::Form(F::PlaceholderShown),
        "valid" => P::Form(F::Valid),
        "invalid" => P::Form(F::Invalid),
        "in-range" => P::Form(F::InRange),
        "out-of-range" => P::Form(F::OutOfRange),
        "hover" => P::User(U::Hover),
        "active" => P::User(U::Active),
        "focus" => P::User(U::Focus),
        "focus-visible" => P::User(U::FocusVisible),
        "focus-within" => P::User(U::FocusWithin),
        "target" => P::User(U::Target),
        _ => return None,
    })
}
