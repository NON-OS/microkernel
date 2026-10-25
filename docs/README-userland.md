# NONOS documentation: Userland

Part of [NONOS documentation](README.md).

### Userland

- [Userland](userland/README.md): what a capsule is, how it goes from source to
  a process, and what lives under `userland/`.
- [Manifests and capabilities](userland/manifests-and-capabilities.md): from
  `Capsule.mk` to a signed manifest to the capability word.
- [Signing and publisher keys](userland/signing-and-publisher-keys.md): the two
  hybrid key pairs, enrollment, and what the kernel refuses.
- [libc and the Rust runtimes](userland/libc.md): the three layers a capsule
  reaches the kernel through.
- [IPC services](userland/ipc-services.md): finding a service by name, who may
  send there, and the message layouts.
- [The prover job](userland/prove-job.md): one PROVE or DELIVER, its stages,
  and when the job exits.
- [The Linux personality](userland/linux-personality.md): how unmodified x86_64
  Linux programs run, and which Linux calls are served.
- [Writing an app](userland/writing-an-app.md): a windowed app on
  `nonos_app_skeleton` and its host tests, step by step on `capsule_hello`.
- [Shipping an app](userland/shipping-an-app.md): the steps from an app crate to
  a signed image and a tile on the dock and the Launchpad.

### ABI

- [The NONOS ABI](abi/README.md): how a call is made, which number names it,
  which capability admits it, and what comes back.
- [Syscalls](abi/syscalls.md): all 130 native syscalls with number, capability
  and meaning.
- [Errors](abi/errors.md): what a failed syscall returns, and every errno the
  kernel defines.
- [Capabilities](abi/capabilities.md): the 37 capability bits and the syscalls
  each one admits.
- [Broker](abi/broker.md): the driver calls to the hardware broker, their
  records and constants.
- [IPC](abi/ipc.md): the IPC calls, the message envelope, the limits and the
  well-known service ports.

### Build

- [Build NONOS](build/README.md): from a clean machine to an image booting under
  QEMU.
- [Toolchain](build/toolchain.md): setting up a build machine, and the pinned
  version of every tool.
- [The Nix flake](build/nix-flake.md): the flake's inputs, packages, apps, shell
  and checks.
- [Make targets](build/make-targets.md): every `make` target and the options a
  QEMU boot takes.
- [Profiles](build/profiles.md): the six build profiles, the keys of
  `nonos.toml`, and boot modes against build profiles.
- [The seal](build/seal.md): what turns unsigned artifacts into a bootable
  image, and what to do without the release keys.
- [Reproducible builds](build/reproducible-builds.md): what is pinned, how to
  compare two builds, and what is not reproducible yet.
- [SBOM](build/sbom.md): the bill of materials and the other supply chain
  records.
- [Host tools](build/host-tools.md): every program under `tools/`, what runs it,
  and how to call it yourself on the build machine.
- [CI](build/ci.md): the GitHub workflows, what `make check` covers, and how the
  checks stood at this commit.

### Contributing

- [Contributing to NONOS](contributing/README.md): set up a checkout, make a
  change, check it and send it.
- [Code style](contributing/code-style.md): the rules a change is held to, and
  which check enforces each one.
- [Commits](contributing/commits.md): the subject line, the body, and what a
  commit leaves out.
- [Tests and proofs](contributing/tests-and-proofs.md): proof crates, static
  checks, Kani, Lean and fuzzing, and which checks fail at this commit.
- [Review](contributing/review.md): what CI runs on a pull request, who reviews
  it, and what reviewers look for.
