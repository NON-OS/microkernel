// NONOS Operating System (AGPL-3.0-or-later)
//! The calls a case should make, stated apart from the interpreter: per
//! connect, the scopes with a region of that space and a _REG loaded, not
//! yet told, in region order; per load, the same for every connected space
//! in the order its first region appears. Region order is table order,
//! then the order devices sit in their table.

use super::gen::{said, Case, Step};

pub fn expect(c: &Case) -> Vec<String> {
    let mut loaded = vec![false; c.ssdts + 1];
    loaded[0] = true;
    let (mut connected, mut told, mut out) = (0u32, Vec::new(), Vec::new());
    let mut order: Vec<usize> = (0..c.devs.len()).collect();
    order.sort_by_key(|i| c.devs[*i].table);
    let mut announce = |loaded: &[bool], space: u8| {
        for d in order.iter().map(|i| &c.devs[*i]).filter(|d| loaded[d.table]) {
            let has_reg = d.reg_in.is_some_and(|t| loaded[t]);
            if d.spaces.contains(&space) && has_reg && !told.contains(&(d.tag, space)) {
                told.push((d.tag, space));
                out.push(said(d.tag, space, 1));
            }
        }
    };
    for step in &c.steps {
        match *step {
            Step::Connect(s) => {
                connected |= 1 << s;
                announce(&loaded, s);
            }
            Step::Load(k, _) => {
                loaded[k] = true;
                let mut spaces = Vec::new();
                for d in order.iter().map(|i| &c.devs[*i]).filter(|d| loaded[d.table]) {
                    for s in &d.spaces {
                        if connected & (1 << s) != 0 && !spaces.contains(s) {
                            spaces.push(*s);
                        }
                    }
                }
                for s in spaces {
                    announce(&loaded, s);
                }
            }
        }
    }
    out
}
