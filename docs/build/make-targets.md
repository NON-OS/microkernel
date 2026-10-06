# Make targets

Look up any `make` target here: what each top-level target runs, the options a QEMU boot takes, and the older `nonos-mk-*` targets in `mk/`.

## How the Makefile works

Every target of the top-level [Makefile](../../Makefile) is one `nix` command, and nothing but Nix is needed to run them (`Makefile:3-4`). With no target, `make` runs `build` (`.DEFAULT_GOAL`, `Makefile:40`). Try the read-only ones first:

```
make help
make -n build
```

`make help` prints the summary at the top of the Makefile (`help`, `Makefile:141-142`). `make -n TARGET` prints the commands a target would run without running them; `make -n build` prints `nix build .#default`, `nix run .#receipt` and the closing `echo`.
