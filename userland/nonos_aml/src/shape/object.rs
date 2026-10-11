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

//! Namespace modifiers and named objects, sections 20.2.5.1 and 20.2.5.2.

use super::{Arg::*, Class::Object, Group};

const fn g(pkg: bool, args: &'static [super::Arg], ops: &'static [u16]) -> Group {
    Group { class: Object, pkg, args, ops }
}

pub const OBJECTS: &[Group] = &[
    // DefScope, DefDevice, DefThermalZone.
    g(true, &[Name, Scope], &[0x10, 0x5B82, 0x5B85]),
    // DefMethod: MethodFlags, then the body.
    g(true, &[Name, U8, Body], &[0x14]),
    // DefPowerRes: SystemLevel, ResourceOrder.
    g(true, &[Name, U8, U16, Scope], &[0x5B84]),
    // DefProcessor: ProcID, PblkAddr, PblkLen.
    g(true, &[Name, U8, U32, U8, Scope], &[0x5B83]),
    // DefField: FieldFlags.
    g(true, &[Name, U8, Fields], &[0x5B81]),
    // DefIndexField: the index field's name, then the data field's.
    g(true, &[Name, Name, U8, Fields], &[0x5B86]),
    // DefBankField: region, bank register, BankValue, FieldFlags.
    g(true, &[Name, Name, Term, U8, Fields], &[0x5B87]),
    g(false, &[Name, Name], &[0x06]),
    g(false, &[Name, Data], &[0x08]),
    // DefExternal: ObjectType, ArgumentCount.
    g(false, &[Name, U8, U8], &[0x15]),
    // DefOpRegion: RegionSpace, RegionOffset, RegionLen.
    g(false, &[Name, U8, Term, Term], &[0x5B80]),
    // DefDataRegion: Signature, OEM ID, OEM Table ID.
    g(false, &[Name, Term, Term, Term], &[0x5B88]),
    // DefMutex: SyncFlags.
    g(false, &[Name, U8], &[0x5B01]),
    g(false, &[Name], &[0x5B02]),
    // DefCreate{Bit,Byte,Word,DWord,QWord}Field: source buffer, index, name.
    g(false, &[Term, Term, Name], &[0x8A, 0x8B, 0x8C, 0x8D, 0x8F]),
    // DefCreateField: source buffer, BitIndex, NumBits, name.
    g(false, &[Term, Term, Term, Name], &[0x5B13]),
];
