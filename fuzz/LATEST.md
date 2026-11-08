# Latest fuzz run

`eb0f76de3b1c`, 2026-11-08T09:13:30+00:00, schedule. https://github.com/NON-OS/microkernel/actions/runs/42463004789

Coverage and corpus against the previous run of each target for the same length of time.

| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 3389155 | 241 | +0 | 432 | +0 | 39 | +0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 2080574591 | 81 | +0 | 83 | +0 | 85 | +0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 198798319 | 303 | +0 | 437 | +0 | 348 | +1 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 239367921 | 339 | +0 | 618 | +0 | 445 | +0 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 5196548 | 1300 | +0 | 4350 | +19 | 6108 | +423 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 91483148 | 479 | +0 | 2558 | +0 | 3474 | +91 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1168068094 | 177 | +0 | 178 | +0 | 43 | +0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1294078227 | 35 | +0 | 36 | +0 | 6 | +0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 343654518 | 107 | +0 | 110 | +0 | 12 | +0 | 0 |
| `userland/net_proofs` | `dns` | pass | 257889501 | 217 | +0 | 844 | +3 | 1603 | +97 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 863470221 | 212 | +0 | 499 | +2 | 1869 | +12 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 3311090 | 2065 | +0 | 4641 | +3 | 685 | +3 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1172591858 | 111 | +0 | 314 | +0 | 510 | +27 | 0 |
