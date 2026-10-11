# tg3_proofs

Host proofs for the tg3 (Broadcom 57765 class) driver capsule
(`capsule_driver_tg3`). The driver's files are compiled here through
`#[path]`, unmodified, over a small libc shim, and run against host memory
for the three DMA grants, a register window (`nonos_devmodel`) and a
modelled PHY on a second thread that answers MDIO frames. The kernel
broker's own `quirk_bits` module is mounted beside them. QEMU has no tg3, so
this is the only run the driver gets before hardware.

## What it proves

62 `#[test]` functions:

- the ten IDs and their 10/100 flags, the revision read from the product-ID
  register, and another class refused before anything is written
  (`chip_tests`);
- the board configuration from SRAM and the LED modes (`hw_cfg_tests`,
  `led_tests`);
- every configuration word, from all 65536 starting values, inside the
  broker's rule, and every write of a bring-up, a chip reset and a stand-down
  replayed through it (`broker_rule_tests`, `config_replay`);
- the return ring against hostile entries, 200,000 random ones included, the
  standard ring's posting, the send ring and its consumer index, the status
  block (`rx_*`, `tx_*`, `status_tests`);
- the mode words and values tg3_reset_hw works out (`init_value_tests`,
  `mode_value_tests`, `init_register_tests`, `abort_tests`);
- MDIO, the PHY reset, copper setup and pause resolution (`mdio_tests`,
  `phy_reset_tests`, `copper_tests`, `relink_tests`, `resolve_tests`);
- the chip reset's order (`reset_tests`), a whole bring-up
  (`bring_up_tests`), the announce line without the address, and a failed
  attempt standing down and giving every grant back (`finish_tests`,
  `stand_down_tests`);
- the request decode and the log line (`decode_fuzz`, `log_tests`).

## What it cannot prove

The SRAM window: a concurrent model cannot answer a read that follows its
address write in time, so the bootcode is absent here and both firmware
waits run out, as on a board without it. Silicon errata, timing, and
anything Linux does that this driver names as not done.

## Run

`cargo test` (about ten seconds: the firmware waits are real).
