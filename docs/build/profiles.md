# Profiles

Pick the kind of image to build here: the six build profiles, the keys of `nonos.toml`, the rules that refuse a weaker image, and how a build profile differs from a boot mode.

## Choose a profile

[nonos.toml](../../nonos.toml) is the configuration of the one build, and every key in it is optional. `nix build` builds what it says; `nix build .#<profile>` builds another [build profile](../overview/glossary.md#build-profile) with the rest of the file unchanged (`profile`, `nonos.toml:1-17`).

```
make profiles
make PROFILE=airgapped
nix build .#core
```

Not tested in this release.

`make profiles` prints, for each profile, what it is, who it is for, its privacy posture and its loader policy (`describe`, `tools/nix/config.nix:193-202`).
