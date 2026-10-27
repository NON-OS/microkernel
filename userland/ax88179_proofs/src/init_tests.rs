// NONOS Operating System (AGPL-3.0-or-later)
//! The bring-up held to the model: every vendor request in order, the drawn
//! address written and the factory one never read, the waits fixed, and a
//! refusal at any step stopping the bind with that step's name.

use nonos_usbnet::mock::{drawn, Call};
use nonos_usbnet::{Bind, Nic, Setup, Taken};

use crate::ax::bind;
use crate::chip::{healthy, Chip, CONFIG, DEVICE};
use crate::harness::{device_of, PAUSED};
use crate::model::{bring_up, Want, PAUSES};
use crate::seen::{vendor, vendor_before};

#[test]
fn the_bring_up_is_the_models_in_order() {
    let (bus, found, _) = device_of(DEVICE, vec![CONFIG.to_vec()], healthy());
    let Bind::Ours(ax) = bind(bus.clone(), &found, drawn()) else { panic!("not bound") };
    let calls = bus.0.borrow().calls.clone();
    assert!(matches!(calls[0], Call::Configure(p) if (p.bulk_in, p.bulk_out) == (0x82, 0x03)));
    assert_eq!(calls[1], Call::Out(Setup::set_configuration(1), vec![]));
    let want: Vec<Want> = bring_up(drawn().bytes()).into_iter().map(|(w, _)| w).collect();
    assert_eq!(vendor(&bus), want);
    assert_eq!(calls.len(), 2 + want.len(), "nothing but the model's requests");
    assert_eq!((ax.mac(), ax.taken(), ax.link_up()), (drawn().bytes(), Taken::Programmed, false));
}

#[test]
fn the_factory_address_is_never_read() {
    let (bus, found, _) = device_of(DEVICE, vec![CONFIG.to_vec()], healthy());
    assert!(matches!(bind(bus.clone(), &found, drawn()), Bind::Ours(_)));
    let reads = vendor(&bus).into_iter().filter(|w| matches!(w, Want::In(1, ..)));
    assert_eq!(reads.count(), 0, "no MAC register is read at all");
    let station = Want::Out(1, 0x10, 6, drawn().bytes().to_vec());
    assert!(vendor(&bus).contains(&station));
}

#[test]
fn the_waits_are_fixed_and_follow_their_steps() {
    let (bus, found, _) = device_of(DEVICE, vec![CONFIG.to_vec()], healthy());
    assert!(matches!(bind(bus.clone(), &found, drawn()), Bind::Ours(_)));
    let paused = PAUSED.with(|p| p.borrow().clone());
    let at: Vec<(u64, usize)> =
        paused.iter().map(|&(ms, n)| (ms, vendor_before(&bus, n))).collect();
    assert_eq!(at, PAUSES);
    assert_eq!(paused.iter().map(|p| p.0).sum::<u64>(), 700);
}

#[test]
fn a_refused_write_stops_the_bind_with_its_name_and_nothing_after_it() {
    let writes: Vec<(Want, &str)> =
        bring_up(drawn().bytes()).into_iter().filter(|(w, _)| matches!(w, Want::Out(..))).collect();
    for (k, (_, name)) in writes.iter().enumerate() {
        let chip = Chip { refuse: Some(k), ..healthy() };
        let (bus, found, _) = device_of(DEVICE, vec![CONFIG.to_vec()], chip);
        match bind(bus.clone(), &found, drawn()) {
            Bind::Failed(what, -32) => assert_eq!(what, *name, "write {k}"),
            _ => panic!("write {k} refused and the bind went on"),
        }
        let outs = vendor(&bus).into_iter().filter(|w| matches!(w, Want::Out(..))).count();
        assert_eq!(outs, k + 1, "write {k}: requests sent after the refusal");
    }
}

#[test]
fn a_phy_control_read_cut_short_stops_the_bind() {
    // The PHY answers one byte where two were asked for.
    let chip = Chip { control: 0x1140, ..healthy() };
    let (bus, found, _) = device_of(DEVICE, vec![CONFIG.to_vec()], chip);
    bus.0.borrow_mut().respond = Box::new(|c| match c {
        Call::In(s, _) if s.request_type == 0xC0 => Ok(vec![0x40]),
        _ => Ok(vec![]),
    });
    assert!(matches!(bind(bus, &found, drawn()), Bind::Failed("PHY control unread", -5)));
}
