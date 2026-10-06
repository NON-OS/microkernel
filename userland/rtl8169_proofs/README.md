# rtl8169_proofs

Host proofs for the RTL8169 bring-up (`capsule_driver_rtl8169`). The crate
includes the shipping driver source with `#[path]`. The register type is built
from a base address, so a window in host memory (`nonos_devmodel`) stands in
for BAR0 and a part modelled from the datasheet answers from it. Calls into
`nonos_libc` land in a shim (`libc_shim/`).

## What it proves

65 `#[test]` functions: the drawn station address, the descriptor rings, the
chip identified from its TxConfig XID against Linux `rtl_chip_infos`, the
8168 and 8125 register maps, the reset and the per-version stop on the clock,
RxConfig, TxConfig and the start per family, the 8168g and 8125 init and start
against modelled ERI and MAC OCP, link speed decode with 2500, and the PCI ids.
See `docs/hardware/rtl8169.md`. The property this crate is
for is the station address: the driver draws one with `CryptoRandom` and never
falls back to the factory address in the part, checked with the entropy source
switched off.

## What it does not prove

The broker, DMA mapping and server halves talk to the kernel and stay out. A
real card's behaviour needs a boot.

## Run

```sh
cd userland/rtl8169_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
