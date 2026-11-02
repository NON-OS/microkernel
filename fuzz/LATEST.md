# Latest fuzz run

`eb0f76de3b1c`, 2026-11-02T09:39:08+00:00, schedule. https://github.com/NON-OS/microkernel/actions/runs/41415182648

Coverage and corpus against the previous run of each target for the same length of time.

| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 3445215 | 241 | +0 | 432 | +0 | 39 | +0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1244479356 | 81 | +0 | 83 | +0 | 85 | +0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 179895366 | 303 | +0 | 437 | +0 | 340 | +0 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 256068119 | 339 | +0 | 618 | +0 | 443 | +2 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 4492665 | 1278 | +4 | 4195 | +38 | 2839 | +699 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 112224251 | 479 | +0 | 2556 | +0 | 2448 | +663 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1475083089 | 177 | +0 | 178 | +0 | 43 | +0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1285970327 | 35 | +0 | 36 | +0 | 6 | +0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 305562483 | 107 | +0 | 107 | +0 | 12 | +0 | 0 |
| `userland/net_proofs` | `dns` | pass | 329486533 | 217 | +0 | 821 | +0 | 1095 | +303 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 827824587 | 208 | +3 | 476 | +17 | 1582 | +277 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 4422631 | 2061 | +0 | 4622 | +2 | 663 | +1 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 891519601 | 110 | +1 | 297 | +3 | 289 | +108 | 0 |
