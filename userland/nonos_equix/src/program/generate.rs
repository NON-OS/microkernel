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


//! Program generation, step for step as the reference does it. Every draw
//! from the stream happens in the same order and under the same conditions,
//! because a single draw out of place yields a different program and so a
//! different hash function, and the service would reject every solution.

use super::schedule::{Ports, TARGET_CYCLE};
use super::template::{Template, BRANCH_MASK, LAYOUT, PORT_NONE};
use super::{Instr, Op, Program, PROGRAM_SIZE};
use crate::siphash::{SipRng, SipState};

const REQUIREMENT_MUL_COUNT: i32 = 192;
const REQUIREMENT_LATENCY: i32 = 195;
const MAX_RETRIES: i32 = 1;
const LOG2_BRANCH_PROB: u32 = 4;

/// R5 cannot be the destination of ADD_RS: on x86 that register needs a
/// displacement in `lea`, and the reference keeps its programs compilable.
const REGISTER_NEEDS_DISPLACEMENT: i32 = 5;

#[derive(Clone, Copy)]
struct Register {
    /// The cycle the register's value is ready in.
    latency: i32,
    /// The group of the last instruction that wrote it, `None` at the start.
    last_op: Option<Op>,
    /// That instruction's parameter; `u32::MAX` means none.
    last_op_par: u32,
}

/// The instruction being built. Operands are signed because the reference
/// marks a missing one with -1, and the destination test compares against it.
struct Draft {
    op: Op,
    src: i32,
    dst: i32,
    imm: u32,
    op_par: u32,
}

struct Ctx {
    cycle: i32,
    sub_cycle: i32,
    mul_count: i32,
    chain_mul: bool,
    latency: i32,
    rng: SipRng,
    registers: [Register; 8],
    ports: Ports,
}

/// The program for one key, or `None` when the key yields a program that is
/// not exactly the required size, multiplication count and latency.
pub fn generate(key: &SipState) -> Option<Program> {
    let mut ctx = Ctx {
        cycle: 0,
        sub_cycle: 0,
        mul_count: 0,
        chain_mul: false,
        latency: 0,
        rng: SipRng::new(*key),
        registers: [Register { latency: 0, last_op: None, last_op_par: u32::MAX }; 8],
        ports: Ports::new(),
    };
    let blank = Instr { op: Op::Target, src: 0, dst: 0, imm: 0 };
    let mut code = [blank; PROGRAM_SIZE];
    let mut size = 0usize;
    let mut attempt = 0;
    let mut last_instr: Option<Op> = None;

    while size < PROGRAM_SIZE {
        let tpl = select_template(&mut ctx, last_instr, attempt);
        last_instr = Some(tpl.group);
        let mut instr = from_template(tpl, &mut ctx.rng);

        let Some(cycle) = ctx.ports.instr(tpl, ctx.cycle, false) else {
            break;
        };
        ctx.chain_mul = attempt > 0;

        let placed = (!tpl.has_src || select_source(tpl, &mut instr, &mut ctx, cycle))
            && (!tpl.has_dst || select_destination(tpl, &mut instr, &mut ctx, cycle));
        if !placed {
            // One retry with the template set narrowed; after that the
            // generator gives up on this cycle and moves to the next one.
            if attempt < MAX_RETRIES {
                attempt += 1;
                continue;
            }
            ctx.sub_cycle += 3;
            ctx.cycle = ctx.sub_cycle / 3;
            attempt = 0;
            continue;
        }
        attempt = 0;

        let Some(cycle) = ctx.ports.instr(tpl, ctx.cycle, true) else {
            break;
        };
        if cycle >= TARGET_CYCLE {
            break;
        }

        if tpl.has_dst {
            let retire = cycle + tpl.latency;
            let reg = &mut ctx.registers[instr.dst as usize];
            reg.latency = retire;
            reg.last_op = Some(tpl.group);
            reg.last_op_par = instr.op_par;
            ctx.latency = ctx.latency.max(retire);
        }

        code[size] = Instr {
            op: instr.op,
            src: instr.src.max(0) as u8,
            dst: instr.dst.max(0) as u8,
            imm: instr.imm,
        };
        size += 1;
        if instr.op.is_mul() {
            ctx.mul_count += 1;
        }
        ctx.sub_cycle += 1;
        if tpl.uop2 != PORT_NONE {
            ctx.sub_cycle += 1;
        }
        ctx.cycle = ctx.sub_cycle / 3;
    }

    // Cycles are numbered from zero, so the last one is the latency less one.
    let uniform = size == PROGRAM_SIZE
        && ctx.mul_count == REQUIREMENT_MUL_COUNT
        && ctx.latency == REQUIREMENT_LATENCY - 1;
    uniform.then_some(Program { code })
}

fn select_template(ctx: &mut Ctx, last_instr: Option<Op>, attempt: i32) -> &'static Template {
    let item = LAYOUT[(ctx.sub_cycle % 36) as usize];
    loop {
        let index = if item.mask0 != 0 {
            let mask = if attempt > 0 { item.mask1 } else { item.mask0 };
            (ctx.rng.u8() & mask) as usize
        } else {
            0
        };
        let tpl = item.templates[index];
        if item.duplicates || Some(tpl.group) != last_instr {
            return tpl;
        }
    }
}

fn branch_mask(rng: &mut SipRng) -> u32 {
    let mut mask = 0u32;
    let mut popcnt = 0;
    while popcnt < LOG2_BRANCH_PROB {
        let bit = 1u32 << (rng.u8() % 32);
        if mask & bit == 0 {
            mask |= bit;
            popcnt += 1;
        }
    }
    mask
}

fn from_template(tpl: &Template, rng: &mut SipRng) -> Draft {
    let mut instr = Draft { op: tpl.op, src: -1, dst: -1, imm: 0, op_par: 0 };
    if tpl.immediate_mask != 0 {
        if tpl.immediate_mask == BRANCH_MASK {
            instr.imm = branch_mask(rng);
        } else {
            loop {
                instr.imm = rng.u32() & tpl.immediate_mask;
                if instr.imm != 0 || tpl.imm_can_be_0 {
                    break;
                }
            }
        }
    }
    if !tpl.op_par_src {
        instr.op_par = if tpl.distinct_dst { u32::MAX } else { rng.u32() };
    }
    instr
}

fn select_register(available: &[i32; 8], count: usize, rng: &mut SipRng) -> Option<i32> {
    match count {
        0 => None,
        1 => Some(available[0]),
        n => Some(available[(rng.u32() % n as u32) as usize]),
    }
}

fn select_source(tpl: &Template, instr: &mut Draft, ctx: &mut Ctx, cycle: i32) -> bool {
    let mut available = [0i32; 8];
    let mut count = 0usize;
    for (i, reg) in ctx.registers.iter().enumerate() {
        if reg.latency <= cycle {
            available[count] = i as i32;
            count += 1;
        }
    }
    // With two candidates for ADD_RS and one of them R5, R5 must be the
    // source, since it can never be the destination.
    if count == 2 && instr.op == Op::AddRs && available[..2].contains(&REGISTER_NEEDS_DISPLACEMENT) {
        instr.src = REGISTER_NEEDS_DISPLACEMENT;
        instr.op_par = REGISTER_NEEDS_DISPLACEMENT as u32;
        return true;
    }
    match select_register(&available, count, &mut ctx.rng) {
        Some(reg) => {
            instr.src = reg;
            if tpl.op_par_src {
                instr.op_par = reg as u32;
            }
            true
        }
        None => false,
    }
}

/// A destination must be ready by `cycle`, must differ from the source where
/// the template asks (no `xor r, r`), must not be multiplied twice running
/// unless a retry allows it, must not repeat the same operation with the same
/// parameter (no `add r, C1; add r, C2`), and must not be R5 for ADD_RS.
fn select_destination(tpl: &Template, instr: &mut Draft, ctx: &mut Ctx, cycle: i32) -> bool {
    let mut available = [0i32; 8];
    let mut count = 0usize;
    for (i, reg) in ctx.registers.iter().enumerate() {
        let i = i as i32;
        let ok = reg.latency <= cycle
            && (!tpl.distinct_dst || i != instr.src)
            && (ctx.chain_mul || tpl.group != Op::MulR || reg.last_op != Some(Op::MulR))
            && (reg.last_op != Some(tpl.group) || reg.last_op_par != instr.op_par)
            && (instr.op != Op::AddRs || i != REGISTER_NEEDS_DISPLACEMENT);
        if ok {
            available[count] = i;
            count += 1;
        }
    }
    match select_register(&available, count, &mut ctx.rng) {
        Some(reg) => {
            instr.dst = reg;
            true
        }
        None => false,
    }
}
