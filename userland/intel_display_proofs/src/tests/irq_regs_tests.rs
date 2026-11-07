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

//! Which interrupt registers each generation's pipes use.

use crate::engine::gen::{GEN11, GEN12, GEN12_3, GEN7_5, GEN8, GEN9};
use crate::engine::irq_regs::{irq_regs, IrqRegs, Master};
use crate::engine::pipe::Pipe;

fn regs(imr: u32, master: Master) -> Option<IrqRegs> {
    Some(IrqRegs { imr, iir: imr + 4, ier: imr + 8, master })
}

#[test]
fn haswell_is_left_polled() {
    assert!(Pipe::ALL.iter().all(|&p| irq_regs(&GEN7_5, p).is_none()));
}

#[test]
fn broadwell_and_gen9_use_the_gen8_master() {
    for g in [&GEN8, &GEN9] {
        assert_eq!(irq_regs(g, Pipe::A), regs(0x4_4404, Master::Gen8));
        assert_eq!(irq_regs(g, Pipe::B), regs(0x4_4414, Master::Gen8));
        assert_eq!(irq_regs(g, Pipe::C), regs(0x4_4424, Master::Gen8));
        assert_eq!(irq_regs(g, Pipe::D), None);
    }
}

#[test]
fn gen11_and_gen12_use_the_split_master() {
    for g in [&GEN11, &GEN12_3] {
        assert_eq!(irq_regs(g, Pipe::C), regs(0x4_4424, Master::Gen11));
        assert_eq!(irq_regs(g, Pipe::D), None);
    }
    assert_eq!(irq_regs(&GEN12, Pipe::D), regs(0x4_4434, Master::Gen11));
}

#[test]
fn the_vblank_iir_is_the_one_the_fault_check_reads() {
    for g in [&GEN8, &GEN9, &GEN11, &GEN12] {
        for p in [Pipe::A, Pipe::B, Pipe::C] {
            let iir = irq_regs(g, p).map(|r| r.iir);
            assert_eq!(iir, crate::engine::fault::de_pipe_iir(g, p));
        }
    }
}
