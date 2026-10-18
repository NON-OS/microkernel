// NONOS Operating System (AGPL-3.0-or-later)
//! A load-time evaluator for the namespace proofs, standing in for the
//! interpreter: an If's predicate may be an integer constant and nothing
//! else, and every other statement is counted, not run.

use crate::arena::Imm;
use crate::exec::{Fault, FaultKind};
use crate::load::{Aml, LoadExec};
use crate::namespace::{Id, ROOT};
use crate::op::Op;

#[derive(Default)]
pub struct Consts {
    pub ran: Vec<Op>,
}

impl LoadExec for Consts {
    fn condition(&mut self, aml: &mut Aml, table: u16, _: Id, node: u32) -> Result<bool, Fault> {
        let t = &aml.tables[table as usize];
        let pred = *t.arena.node(t.arena.kids(node)[0]).unwrap();
        match (pred.op, pred.imm) {
            (Op::ZERO | Op::ONE | Op::ONES, Imm::Int(v)) => Ok(v != 0),
            (op, Imm::Int(v)) if op.0 <= 0x0E => Ok(v != 0),
            _ => Err(Fault::new(FaultKind::Type, table, pred.at)),
        }
    }

    fn run(&mut self, aml: &mut Aml, table: u16, _: Id, node: u32) -> Result<(), Fault> {
        let n = *aml.tables[table as usize].arena.node(node).unwrap();
        self.ran.push(n.op);
        if n.op == Op::FATAL {
            return Err(Fault::new(FaultKind::Fatal, table, n.at));
        }
        Ok(())
    }
}

/// Loads `dsdt`, then each SSDT, into a fresh namespace.
pub fn load(dsdt: &[u8], ssdts: &[Vec<u8>]) -> (Aml, Consts) {
    let mut aml = Aml::default();
    let mut exec = Consts::default();
    aml.load_table(crate::enc::table(b"DSDT", 2, dsdt), &mut exec).expect("DSDT loads");
    for s in ssdts {
        aml.load_table(crate::enc::table(b"SSDT", 2, s), &mut exec).expect("SSDT loads");
    }
    (aml, exec)
}

/// The entry at an ASL path, from the root.
pub fn at(aml: &Aml, path: &str) -> Option<Id> {
    let bytes = crate::enc::name(path);
    let mut c = crate::grammar::Cursor::new(&bytes, 0, bytes.len() as u32);
    let p = crate::grammar::name_string(&mut c).unwrap();
    aml.ns.resolve(ROOT, &bytes, p)
}
