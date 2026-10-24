# smbus_proofs

Host proofs for the Intel i801 SMBus host capsule (`capsule_driver_smbus`).
The crate mounts the shipping registers, transfer engine, request parsing,
HOSTC rule and answers with `#[path]` and runs them against a model of the
host: its registers, the INUSE semaphore, write-one-to-clear status, the
32-byte block buffer and its index, and EEPROM-like devices at their
addresses.

## What it proves

26 `#[test]` functions:

- byte, byte data, word data and quick transfers, and the registers each
  writes (`transfer_tests`);
- block reads and writes through the buffer, a bad length, and a host
  without the buffer (`block_tests`);
- a missing device, a busy host, leftover flags, which error bit is
  answered first, a host idle before its result shows, and a timeout with
  its kill (`error_tests`);
- each protocol's plan: command, data registers, address byte
  (`plan_tests`), and the status read as running, finished or a fault
  (`status_tests`);
- what a request may carry (`parse_tests`);
- HOSTC (`hostc_tests`);
- the answers (`answer_tests`).

## Run

```sh
cd userland/smbus_proofs && cargo test --release
```
