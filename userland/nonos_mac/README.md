# nonos_mac

`nonos_mac` turns random bytes into a locally administered unicast MAC
address, so a network driver transmits from that instead of the factory
address in its EEPROM or efuse. One crate for every driver, so they all apply
the same rule. `no_std` outside its own tests, no dependencies.

## Public surface

- `apply(&mut [u8; 6])`: sets the locally administered bit (0x02) and clears
  the group bit (0x01) of the first octet. The other 46 bits are left as they
  are. Drivers call this form because returning an array across the crate
  boundary emits a `memcpy` that a freestanding capsule cannot link.
- `from_random([u8; 6]) -> [u8; 6]`: the same rule, by value.
- `is_local_unicast(&mac)`: group bit clear and local bit set.
- `is_factory_assigned(&mac)`: local bit clear.
- `MAC_LEN`, 6.

## What it does not do

It has no source of randomness. The caller passes the bytes, and the address
is only as unpredictable as those bytes. It does not decide when to rotate an
address or read the factory address from hardware.

## Users

`capsule_driver_e1000`, `capsule_driver_iwlwifi`, `capsule_driver_rtl8139`,
`capsule_driver_rtl8169`, `capsule_driver_rtl8821ce` and
`capsule_driver_virtio_net` call `apply`. `e1000_proofs` and `rtl8169_proofs`
check the driver's result with `is_local_unicast`, and `rtl8139_proofs` depends
on it because it compiles the driver's sources.

## Tests

- Seven unit tests in `src/tests.rs`: a real vendor address is recognised and
  rewritten, only the first octet moves, multicast, broadcast and all-zero
  inputs come out local unicast, and applying twice changes nothing.
- Six Kani harnesses in `src/proofs.rs` (`#[cfg(kani)]`) over all six octets:
  the result is always local unicast, keeps every other bit, never equals a
  factory address, broadcast or zero, and the rule is idempotent.

Neither runs in `nix flake check`, since this is not a `*_proofs` crate, and
the Kani job in `.github/workflows/verify.yml` does not list it. Run them with
`cargo test` and `cargo kani` from this directory. The driver-side checks run
in `proofs-e1000_proofs` and `proofs-rtl8169_proofs`. See
[Drivers](../../docs/handbook/drivers.md).
