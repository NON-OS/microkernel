// NONOS Operating System (AGPL-3.0-or-later)
//! A `/cpus` with `idle-states`, built to order, and the two readers run
//! over it the way boot runs them.

use crate::arch::fdt::find::cpus::idle_lists;
use crate::arch::fdt::find::idle::{states, CpuStates, IdleState, Refusal, MAX_STATES};
use crate::arch::fdt::Fdt;
use crate::builder::{cells, text, Builder, END};

/// One child of `idle-states`: its name and properties, in order.
pub type Node<'a> = (&'a str, Vec<(&'a str, Vec<u8>)>);

/// A complete standby state: phandle, param, entry, exit, residency.
pub fn standby(name: &str, ph: u32, param: u32, res: u32) -> Node<'_> {
    let mut props = vec![("compatible", text("arm,idle-state"))];
    props.push(("phandle", cells(&[ph])));
    props.push(("arm,psci-suspend-param", cells(&[param])));
    props.push(("entry-latency-us", cells(&[10])));
    props.push(("exit-latency-us", cells(&[20])));
    props.push(("min-residency-us", cells(&[res])));
    (name, props)
}

/// `method` is `entry-method`; `cpus` holds each CPU's `cpu-idle-states`
/// (`None`: no `reg`, so the CPU is not one `find` lists).
pub fn tree(method: Option<&str>, nodes: &[Node], cpus: &[(Option<u32>, Vec<u8>)]) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[2]));
    b = b.begin("cpus").prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[0]));
    for (i, (reg, list)) in cpus.iter().enumerate() {
        b = b.begin(&format!("cpu@{i}")).prop("device_type", &text("cpu"));
        if let Some(reg) = reg {
            b = b.prop("reg", &cells(&[*reg]));
        }
        if !list.is_empty() {
            b = b.prop("cpu-idle-states", list);
        }
        b = b.end();
    }
    b = b.begin("idle-states");
    if let Some(method) = method {
        b = b.prop("entry-method", &text(method));
    }
    for (name, props) in nodes {
        b = b.begin(name);
        for (key, value) in props {
            b = b.prop(key, value);
        }
        b = b.end();
    }
    b.end().end().end().token(END).finish()
}

/// What boot keeps: the states, then each CPU's indices into them.
pub type Kept = (Vec<IdleState>, Vec<Vec<u8>>);

pub fn read(blob: &[u8]) -> Result<Kept, Refusal> {
    let fdt = Fdt::from_bytes(blob).map_err(Refusal::Tree)?;
    let mut found = [IdleState::default(); MAX_STATES];
    let n = states(&fdt, &mut found)?;
    let mut lists = [CpuStates::default(); 8];
    let m = idle_lists(&fdt, &found[..n], &mut lists)?;
    let lists = lists[..m].iter().map(|l| l.indices().to_vec()).collect();
    Ok((found[..n].to_vec(), lists))
}
