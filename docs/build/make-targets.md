# Make targets

Look up any `make` target here: what each top-level target runs, the options a QEMU boot takes, and the older `nonos-mk-*` targets in `mk/`.

## How the Makefile works

Every target of the top-level [Makefile](../../Makefile) is one `nix` command, and nothing but Nix is needed to run them (`Makefile:3-4`). With no target, `make` runs `build` (`.DEFAULT_GOAL`, `Makefile:40`). Try the read-only ones first:

```
make help
make -n build
```

`make help` prints the summary at the top of the Makefile (`help`, `Makefile:141-142`). `make -n TARGET` prints the commands a target would run without running them; `make -n build` prints `nix build .#default`, `nix run .#receipt` and the closing `echo`.

## Build and check

| target | what it does |
|---|---|
| `build`, the default | `nix build .#default`, or `.#<profile>` when `PROFILE` is set, then `nix run .#receipt`, which prints the build receipt and compares it with the receipt committed for that profile (`build`, `Makefile:53-56`) |
| `check` | `nix run .#check-report`: builds every flake check for this host and prints what each proved (`check`, `Makefile:60-61`) |
| `profiles` | prints each [build profile](../overview/glossary.md#build-profile), what it is for, its privacy posture and its loader policy (`profiles`, `Makefile:101-102`) |
| `shell` | `nix develop`, the pinned toolchain for work by hand (`shell`, `Makefile:104-105`) |
| `doctor` | checks for Nix, flakes and hardware virtualization (`doctor`, `Makefile:124-136`) |
| `clean` | removes `result`, every `result-*` link and the whole `target` directory (`clean`, `Makefile:138-139`) |
| `help` | prints the Makefile's own summary (`help`, `Makefile:141-142`) |

`make clean` also removes what lives under `target`: sealed images, the development checkout and its throwaway keys, the QEMU data disk, the software TPM state and the Qwen files fetched into `target/models/files`.

The receipt step writes too. It replaces the committed receipt for that profile in your checkout, except when the build failed to reproduce, and it exits with an error when the same commit and inputs gave other bytes or when the kernel holds a capsule its profile takes out (`main`, `tools/nonos-receipt:243-263`). [reproducible-builds.md](reproducible-builds.md) explains the verdicts.
