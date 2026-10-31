# Latest fuzz run

`eb0f76de3b1c`, 2026-10-31T19:54:16+00:00, push. https://github.com/NON-OS/microkernel/actions/runs/41225526739

Coverage and corpus against the previous run of each target for the same length of time.

| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 291358 | 241 |  | 432 |  | 39 |  | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 83343781 | 81 |  | 83 |  | 85 |  | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 17314461 | 303 |  | 437 |  | 340 |  | 0 |
| `userland/admission_proofs` | `manifest` | pass | 23532584 | 339 |  | 618 |  | 438 |  | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 503392 | 1265 |  | 4125 |  | 1443 |  | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 3008745 | 479 |  | 2548 |  | 548 |  | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 130960869 | 177 |  | 178 |  | 43 |  | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 95111904 | 35 |  | 36 |  | 6 |  | 0 |
| `userland/market_proto` | `linux_policy` | pass | 22381298 | 107 |  | 107 |  | 12 |  | 0 |
| `userland/net_proofs` | `dns` | pass | 16756583 | 213 |  | 805 |  | 438 |  | 0 |
| `userland/net_proofs` | `net_packets` | pass | 52717076 | 204 |  | 457 |  | 584 |  | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 388769 | 2060 |  | 4613 |  | 658 |  | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 83680194 | 104 |  | 228 |  | 115 |  | 0 |
