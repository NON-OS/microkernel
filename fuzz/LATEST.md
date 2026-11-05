# Latest fuzz run

`eb0f76de3b1c`, 2026-11-05T09:29:48+00:00, schedule. https://github.com/NON-OS/microkernel/actions/runs/41997746574

Coverage and corpus against the previous run of each target for the same length of time.

| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 3270040 | 241 | +0 | 432 | +0 | 39 | +0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1252894481 | 81 | +0 | 83 | +0 | 85 | +0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 292234653 | 303 | +0 | 437 | +0 | 347 | +4 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 203657869 | 339 | +0 | 618 | +0 | 445 | +1 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 6685380 | 1295 | +1 | 4301 | +20 | 4878 | +599 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 71676381 | 479 | +0 | 2557 | +1 | 3093 | +168 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1997403539 | 177 | +0 | 178 | +0 | 43 | +0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1857103052 | 35 | +0 | 36 | +0 | 6 | +0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 301730623 | 107 | +0 | 107 | +0 | 12 | +0 | 0 |
| `userland/net_proofs` | `dns` | pass | 175387983 | 217 | +0 | 836 | +3 | 1370 | +65 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 800278042 | 212 | +2 | 494 | +7 | 1804 | +26 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 4006076 | 2065 | +1 | 4633 | +3 | 676 | +6 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1156326328 | 111 | +0 | 306 | +3 | 450 | +36 | 0 |
