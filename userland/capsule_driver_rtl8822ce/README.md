# capsule_driver_rtl8822ce

## Role

`capsule_driver_rtl8822ce` is the Realtek RTL8822CE PCIe Wi-Fi hardware
capsule (rtw88's 8822C, two RF paths, PCI 10ec:c822 and 10ec:c82f). It owns
the device claim, BAR mapping, DMA rings, firmware download, the MAC/PHY/RF
bring-up, scanning and joining for `driver.rtl8822ce0`.

```text
net.core, Settings, setup wizard (the only senders the kernel admits)
 |
 v
driver.rtl8822ce0 -- brokered MMIO/DMA, polled --> Realtek RTL8822CE PCIe device
```

It is the RTL8821CE driver with the chip's own parts replaced. The serve
loop, the wire protocol, the station address, the rings and descriptors,
the H2C path, the firmware download engine, scanning, joining, security
and the link protocol are the 8821CE capsule's own files, mounted by
`#[path]` and compiled unchanged; `src/main.rs` and each directory's
`mod.rs` say which are shared and which are this chip's. Nothing in
`capsule_driver_rtl8821ce` was edited for this driver, so the 8821CE (the
owner's laptop chip) cannot regress through it. Its log lines are the
shared ones re-tagged `[rtl8822ce]` as each whole line is assembled
(`src/status/line.rs`).

## Why a second capsule and not a generalised 8821CE

The two chips share the PCIe glue, the descriptor formats, the firmware
download protocol and the 802.11 side. They differ in every table and in
the PHY: another power sequence, another efuse layout, a FIFO four times
the size, two RF paths behind other windows, a DAC calibration the 8821C
does not have, another TX power scale and another PHY status layout
([docs/differences.md](docs/differences.md)). Generalising would have put a
chip switch through code the 8821CE runs every boot; a capsule of its own
keeps that code byte-identical and the 8821CE's proofs untouched.

## Not done yet (bounded, and said on the log)

IQK, DPK and TX gain calibration, the general and PHY-DM info H2Cs, DIG,
thermal power tracking, beamforming, the power-by-rate and power-limit
tables, and 5 GHz. The 8821CE driver has none of these either. The
receive and transmit paths run uncalibrated beyond the DAC calibration and
the factory trims: expect shorter range than under Linux until a hardware
run says what is worth porting next ([docs/hardware.md](docs/hardware.md)).

## Licence of what came from rtw88

The register values, sequences and tables come from Linux v6.12
`drivers/net/wireless/realtek/rtw88/`, which is dual-licensed
"GPL-2.0 OR BSD-3-Clause". NONOS takes it under the **BSD-3-Clause** option.
The files read: `rtw8822c.c`, `rtw8822c.h`, `rtw8822c_table.c`, `reg.h`,
`mac.c`, `phy.c`, `coex.c`, `pci.c` and `efuse.c`.

- The five PHY tables (`src/phy/tables/bb.rs`, `agc.rs`, `cal_init.rs`,
  `rf_a.rs`, `rf_b.rs`) are copied data, written by `tools/nonos-rtw88-tables`
  from `rtw8822c_table.c` (sha256 pinned in the tool); each file carries
  Realtek's copyright and the full BSD-3-Clause notice beside the table.
- The power sequences, MAC init table, FIFO layout and calibration steps
  are rewritten in Rust from those files, under the same option; the
  source file and function are named in each module's comment.

The firmware is a separate binary with its own licence:
[firmware/README.md](firmware/README.md).

## Parts

- [What differs from the RTL8821CE](docs/differences.md)
- [Real hardware: what is missing, what to look at](docs/hardware.md)
- [Verification](docs/verification.md)
- [Firmware](firmware/README.md)
