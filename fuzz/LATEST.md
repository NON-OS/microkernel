# Latest fuzz run

`eb0f76de3b1c`, 2026-11-04T09:22:15+00:00, schedule. https://github.com/NON-OS/microkernel/actions/runs/41799666571

Coverage and corpus against the previous run of each target for the same length of time.

| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 3250195 | 241 | +0 | 432 | +0 | 39 | +0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1234991746 | 81 | +0 | 83 | +0 | 85 | +0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 166131635 | 303 | +0 | 437 | +0 | 343 | +2 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 213623851 | 339 | +0 | 618 | +0 | 444 | +1 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 4654801 | 1294 | +8 | 4281 | +24 | 4279 | +575 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 62319472 | 479 | +0 | 2556 | +0 | 2925 | +191 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1142028864 | 177 | +0 | 178 | +0 | 43 | +0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1273864517 | 35 | +0 | 36 | +0 | 6 | +0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 289216818 | 107 | +0 | 107 | +0 | 12 | +0 | 0 |
| `userland/net_proofs` | `dns` | pass | 255306041 | 217 | +0 | 833 | +12 | 1305 | +59 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 820653893 | 210 | +0 | 487 | +8 | 1778 | +60 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 3072124 | 2064 | +3 | 4630 | +3 | 670 | +3 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 764885099 | 111 | +0 | 303 | +0 | 414 | +50 | 0 |
