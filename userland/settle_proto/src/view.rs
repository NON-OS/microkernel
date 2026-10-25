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

//! READ's views (plan/28): each names one config contract's view, its
//! selector and its argument words. The caller names a view and supplies the
//! words; settle builds the call, so a caller never chooses `to` or calldata.

use nonos_nox_config::Contract;

/// The form one argument word must have.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arg {
    /// An address: the top 12 bytes are zero.
    Address,
    /// Any 32 bytes.
    Word,
    /// A uint8.
    U8,
    /// A uint64.
    U64,
    /// A bool: 0 or 1.
    Bool,
}

impl Arg {
    pub fn fits(self, w: &[u8; 32]) -> bool {
        let zero = match self {
            Arg::Address => 12,
            Arg::Word => 0,
            Arg::U8 | Arg::Bool => 31,
            Arg::U64 => 24,
        };
        let clear = w.get(..zero).is_some_and(|z| z.iter().all(|b| *b == 0));
        clear && (self != Arg::Bool || w.get(31).is_some_and(|b| *b <= 1))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct View {
    pub id: u16,
    pub contract: Contract,
    pub signature: &'static str,
    pub selector: [u8; 4],
    pub args: &'static [Arg],
}

/// The view numbered `id`, if any.
pub fn view(id: u16) -> Option<&'static View> {
    crate::views::VIEWS.iter().find(|v| v.id == id)
}

/// Its calldata, when `args` are the view's own: the right count, each in
/// its form.
pub fn view_calldata(v: &View, args: &[[u8; 32]]) -> Option<alloc::vec::Vec<u8>> {
    if args.len() != v.args.len() || !v.args.iter().zip(args).all(|(a, w)| a.fits(w)) {
        return None;
    }
    let mut out = alloc::vec::Vec::with_capacity(args.len().saturating_mul(32).saturating_add(4));
    out.extend_from_slice(&v.selector);
    args.iter().for_each(|w| out.extend_from_slice(w));
    Some(out)
}
