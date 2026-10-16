# Architecture: Counting the tree

Part of [Architecture](architecture.md).

## Counting the tree

The counts on this page come from these commands, run at this commit from the
repository root:

```sh
find src -name '*.rs' | wc -l                                  # 5761
find src -name '*.rs' -print0 | xargs -0 cat | wc -l           # 300321
ls -d userland/*/Cargo.toml | wc -l                            # 271
ls userland/*/Capsule.mk | wc -l                               # 107
ls -d userland/capsule_driver_*/ | wc -l                       # 27
ls userland/capsule_driver_*/Capsule.mk | wc -l                # 26
grep -c 'tag4(b"' src/syscall/numbers/defs.rs                  # 130
grep -c ' = 1 << ' src/capabilities/types/defs.rs              # 37
python3 -c "import json; print(len(json.load(open('tools/nix/capsules.json'))))"   # 116
python3 -c "import json; print(sum(e['slug'].startswith('driver-') for e in json.load(open('tools/nix/capsules.json'))))"   # 18
python3 -c "import json; print(sum(e['dir'] == 'userland/linux_userland' for e in json.load(open('tools/nix/capsules.json'))))"   # 19
```

| What | Count |
|---|---|
| Rust files under `src/` | 5,761 |
| Lines in those files | 300,321 |
| Crates directly under `userland/` (directories with a `Cargo.toml`) | 271 |
| Directories under `userland/` with a `Capsule.mk` | 107 |
| Driver capsule directories, `userland/capsule_driver_*` | 27 |
| Those with a `Capsule.mk` | 26 |
| Entries in the build catalogue `tools/nix/capsules.json` | 116 |
| Driver capsules in that catalogue | 18 |
| Linux userland programs in that catalogue | 19 |
| Syscalls | 130 |
| Capability bits | 37 |

## See also

- [Overview](README.md)
- [Design principles](design-principles.md)
- [Threat model](threat-model.md)
- [Kernel](../kernel/README.md)
- [Drivers](../drivers/README.md)
- [Userland](../userland/README.md)
- [Security](../security/README.md)
