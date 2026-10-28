// NONOS Operating System (AGPL-3.0-or-later)
//! Bus mastering comes on only after the reset and this driver's ring: the
//! claim asks for decode alone, and the MASTER command follows the handshake
//! (nonos_virtio's `start` resets first) and comes before `ready` hands the
//! ring over. The shipping setup is read as written, since it calls the
//! broker and no host model of it exists.

const SEQUENCE: &str = include_str!("../../../capsule_driver_virtio_input/src/setup/sequence.rs");
const PCI: &str = include_str!("../../../capsule_driver_virtio_input/src/setup/pci.rs");

fn at(needle: &str) -> usize {
    SEQUENCE.find(needle).unwrap_or_else(|| panic!("sequence.rs has no {needle}"))
}

#[test]
fn bus_mastering_follows_the_reset_and_the_ring() {
    assert_eq!(SEQUENCE.matches("pci::command(").count(), 2, "one decode, one master");
    let decode = at("pci::command(dev.device_id, epoch, pci::DECODE)");
    let ring = at("handshake::run(");
    let master = at("pci::command(dev.device_id, epoch, pci::MASTER)");
    let ready = at("ready::finish(");
    assert!(decode < ring && ring < master && master < ready);
}

#[test]
fn decode_alone_holds_no_bus_master_bit() {
    let decode = PCI.split("pub const DECODE: u16 =").nth(1).expect("DECODE");
    let decode = decode.split(';').next().expect("one line");
    assert!(!decode.contains("BUS_MASTER"), "DECODE is {decode}");
}
