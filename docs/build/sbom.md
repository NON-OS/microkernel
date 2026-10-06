# SBOM

Get the NONOS bill of materials here, see what it lists and what it leaves out, and find the other records of what the build is made of.

## Get it

```
nix build .#sbom
jq '.components | length' result
```

Not tested in this release.

`nix build .#sbom` writes `nonos.cdx.json`, a CycloneDX 1.5 document (`bom`, `tools/nix/sbom.nix:53-67`). The same file sits in every build's `result/` (`sbom`, `tools/nix/artifacts.nix:66`), is published with every release (`assets`, `.github/workflows/ci-release-artifacts.yml:60-64`), and is kept with the reports of the supply chain workflow, which fails when it is not CycloneDX or lists no component (`jq`, `.github/workflows/ci-supply-chain.yml:37-44`). `make check` ends by printing how many components it lists and how many are pinned by hash (`sbom_out`, `tools/nonos-check-report:92-110`).

It is computed from the pins themselves, the `lockFiles` and the other pin files, not from what one run happened to fetch, so it is the same file on every machine (`tools/nix/sbom.nix:1-6`). No SPDX document is produced.
