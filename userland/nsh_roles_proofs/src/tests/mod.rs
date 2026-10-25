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

//! The proofs, one module per part of docs/design/nsh-v1-roles.md.

mod approvals_direct;
mod approvals_order;
mod escalate;
mod escalate_approval_sig;
mod escalate_approvals;
mod escalate_caps;
mod escalate_issuer;
mod escalate_parent;
mod escalate_scope;
mod escalate_time;
mod host;
mod kernel_names;
mod login_grant;
mod login_key;
mod login_list;
mod login_order;
mod login_user;
mod policy;
mod policy_approvers;
mod policy_ceilings;
mod policy_keys;
mod policy_role_fields;
mod policy_roles;
mod policy_shape;
mod policy_sig;
mod reasons;
mod replay;
mod revoke;
mod revoke_sig;
mod shipped;
mod shipped_table;
