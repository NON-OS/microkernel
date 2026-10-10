# The kernel, the bootloader and the host tools the seal runs.
#
# Each compiled artifact embeds only what the tree holds: the kernel embeds the
# capsule ELFs built here and the trust set committed under nonos-data/trust,
# and the bootloader embeds the kernel root and the kernel's public keys
# committed there. Enrollment and signing write those files; the flake reads
# them. So the artifacts are a function of the source, and a sealed release is
# these artifacts plus what the seal added to them.
{ pkgs, lib, pins, rustBuild, src, capsules, self }:
let
  llvm = pins.llvm;

  # The kernel's crates and what they name (tools/nix/inputs.py), its target,
  # and the script that lists its unenforced controls into the output.
  kernelSource = src.crate "." [
    "x86_64-nonos.json"
    "scripts/check_unenforced.py"
    "scripts/gate.py"
    "scripts/unenforced_scan.py"
    "scripts/unreachable_scan.py"
  ];

  # The kernel's build script asks git for the commit it names in its build
  # identity. There is no repository in the sandbox, so this answers with the
  # commit the flake was evaluated at.
  rev = self.shortRev or self.dirtyShortRev or "unknown";
  gitShim = pkgs.writeShellScriptBin "git" ''
    case "$*" in
      "rev-parse --short HEAD") echo ${rev} ;;
      *) echo "git $* is not available in the build sandbox" >&2; exit 1 ;;
    esac
  '';

  # The build time every artifact carries: the release date in
  # tools/nix/source-date-epoch, never the clock's and never the commit's. The
  # kernel signs it into its manifest, so the commit time made every commit,
  # a docs one included, a different kernel: the image a seal made on one
  # commit could not match a build of the commit that records it.
  epoch = lib.removeSuffix "\n" (builtins.readFile ./source-date-epoch);

  # The kernel's build script signs a legacy manifest section with
  # NONOS_SIGNING_KEY and refuses a release build without one. Nothing reads
  # that section (the loader checks the signature the seal adds), so the build
  # passes the published CI placeholder from nonos-ci/setup-signing-key.sh:
  # not a secret, and the same on every machine, so the kernel stays
  # reproducible and no build ever needs a key.
  placeholder = ''
    printf 'NONOS-CI-FORK-DEV-SEED-32-BYTE!!' > $TMPDIR/build-placeholder.seed
    export NONOS_SIGNING_KEY=$TMPDIR/build-placeholder.seed
  '';

  # Puts each capsule the kernel embeds where its include_bytes! reads it.
  # Every capsule the kernel's features embed, where the kernel's
  # include_bytes! reads it. The crates.io tools in the tool registry
  # (src/userspace/tool_capsules/registry.rs) are embedded under
  # nonos-tool-capsules, not their own features, from target/upstream-*/bin.
  registryTool = e: builtins.match "target/upstream-[^/]+/bin/.*" e.prebuilt != null;
  shipped =
    enabled:
    builtins.filter (e: builtins.elem e.feature enabled
      || (registryTool e && builtins.elem "nonos-tool-capsules" enabled)) capsules.catalogue;

  # The image's capability ceiling: every bit a capsule this profile ships
  # declares in its own ceiling, and no other. A profile that leaves out the
  # Linux personality cannot hand a capsule ForeignExec; one that ships About
  # holds AttestRead. The kernel bakes it (build.rs, src/security/image_ceiling).
  ceilingOf =
    enabled: lib.foldl' (acc: e: lib.bitOr acc (lib.fromHexString e.caps_ceiling)) 0 (shipped enabled);

  placeCapsules =
    enabled:
    lib.concatMapStrings (e: ''
      mkdir -p ${e.dir}/target/${e.target}/release
      cp ${capsules.bySlug.${e.slug}}/${e.bin} ${e.dir}/target/${e.target}/release/${e.bin}
      ${lib.optionalString (e.prebuilt != "") ''
        mkdir -p $(dirname ${e.prebuilt})
        cp ${capsules.bySlug.${e.slug}}/${e.bin} ${e.prebuilt}
      ''}
    '') (builtins.filter (e: builtins.elem e.feature enabled
      || (registryTool e && builtins.elem "nonos-tool-capsules" enabled)) capsules.catalogue);

  # A configuration nonos.toml cannot have yet fails when it is built, saying
  # why, rather than when the flake is evaluated, so every other output and
  # check still evaluates.
  refuse =
    cfg: what:
    pkgs.runCommand "nonos-${what}-${cfg.name}-refused" { } ''
      echo ${lib.escapeShellArg cfg.blocked} >&2
      exit 1
    '';

  kernel = cfg: if cfg.blocked != null then refuse cfg "kernel" else kernelFor cfg;

  # The kernel embeds each shipped capsule's certificate, manifest and STARK
  # trailer from nonos-data/trust, which the seal writes and ek commits. A
  # capsule added since the last seal has none yet, so the kernel cannot be
  # built until the next seal; these are the capsules waiting for it.
  unsealed =
    enabled:
    map (e: e.slug) (builtins.filter (e: !(lib.all (f: builtins.pathExists (src.root + "/${f}")) [ e.cert e.manifest e.trailer ]))
      (shipped enabled));
  kernelReady = cfg: unsealed cfg.enabled == [ ];

  kernelFor =
    cfg:
    rustBuild {
      name = "nonos-kernel-${cfg.name}";
      src = kernelSource;
      lockFiles = [ (src.root + "/Cargo.lock") ];
      buildStd = true;
      nativeBuildInputs = [ llvm.clang-unwrapped llvm.llvm gitShim ];
      script = ''
        ${lib.optionalString (!kernelReady cfg) ''
          echo "the kernel embeds the trust set of every capsule it ships, and the tree has none for: ${toString (unsealed cfg.enabled)}; the seal writes it (tools/nix/README.md, Seal)" >&2
          exit 1
        ''}
        ${placeCapsules cfg.enabled}
        ${placeholder}
        python3 -c 'import struct; open("nonos-data/trust/policy/image_capability_ceiling.bin", "wb").write(struct.pack("<Q", ${toString (ceilingOf cfg.enabled)}))'
        # build.rs compiles the arch C with "clang" by name; on macOS the first
        # clang is the stdenv wrapper and adds host flags, so the kernel differed
        # between a macOS and a Linux host. Unwrapped first, as for the loader.
        export PATH=${llvm.clang-unwrapped}/bin:$PATH
        export SOURCE_DATE_EPOCH=${epoch} NONOS_USER_TARGET=x86_64-nonos-user
        export LLVM_AR=${llvm.llvm}/bin/llvm-ar LLVM_RANLIB=${llvm.llvm}/bin/llvm-ranlib
        cargo build --frozen --release --target x86_64-nonos.json \
          -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem \
          --no-default-features --features ${lib.concatStringsSep "," cfg.features}
        mkdir -p $out
        cp target/x86_64-nonos/release/nonos-kernel $out/nonos-kernel
        python3 scripts/check_unenforced.py --list > $out/unenforced.txt
      '';
    };

  # The public halves the bootloader compiles in. The kernel signing keys live
  # with ek (nonos-bootloader/keys/, never committed); the seal copies their
  # .pub files here so a build of the loader needs no key, only the tree.
  kernelKeys = {
    ed25519 = "nonos-data/trust/keys/kernel_signing_ed25519.pub";
    mldsa65 = "nonos-data/trust/keys/kernel_mldsa65.pub";
    root = "nonos-data/trust/policy/kernel_attest_root.bin";
  };

  bootloader =
    cfg:
    rustBuild {
      name = "nonos-bootloader-${cfg.loader}";
      # The loader's crates, and the kernel keys the seal commits for it.
      src = src.crate "nonos-bootloader" (builtins.filter (f: builtins.pathExists (src.root + "/${f}")) (builtins.attrValues kernelKeys));
      lockFiles = [ (src.root + "/nonos-bootloader/Cargo.lock") ];
      buildStd = false;
      rust = pins.rustUefi;
      nativeBuildInputs = [ llvm.clang-unwrapped llvm.llvm ];
      script = ''
        # build.rs asks for "clang" by name to cross-compile the ML-DSA C for
        # UEFI. On macOS the first clang is the stdenv wrapper, which adds host
        # flags (-fPIC) the UEFI target refuses; the unwrapped one goes first,
        # as it already is on Linux.
        export PATH=${llvm.clang-unwrapped}/bin:$PATH
        for f in ${kernelKeys.ed25519} ${kernelKeys.mldsa65} ${kernelKeys.root}; do
          [ -f "$f" ] || { echo "the bootloader compiles in $f, and the tree has none: the seal writes it (tools/nix/README.md, Seal)" >&2; exit 1; }
        done
        tail -c 32 ${kernelKeys.ed25519} > $TMPDIR/kernel_signing_ed25519.raw
        export SOURCE_DATE_EPOCH=${epoch}
        export NONOS_TRUST_ANCHOR_PUBKEY=$TMPDIR/kernel_signing_ed25519.raw
        export NONOS_MLDSA65_PUBKEY=$PWD/${kernelKeys.mldsa65}
        export NONOS_KERNEL_ATTEST_ROOT=$PWD/${kernelKeys.root}
        cd nonos-bootloader
        RUSTFLAGS='-C panic=abort -C target-feature=+crt-static --cfg curve25519_dalek_backend="serial" -C link-arg=/DEBUG:NONE' \
          cargo build --frozen --release --target x86_64-unknown-uefi --features ${cfg.loader}
        mkdir -p $out
        cp target/x86_64-unknown-uefi/release/nonos_boot.efi $out/nonos_boot.efi
      '';
    };

  # The host programs the seal and the checks run, built once for the build
  # machine. Each is the same crate and lock the make rules build.
  hostTool =
    { name, dir, bins, args ? "" }:
    rustBuild {
      name = "nonos-host-${name}";
      src = src.crate dir [ ];
      lockFiles = [ (src.root + "/${dir}/Cargo.lock") ];
      # sign-kernel's HTTP client links the host's OpenSSL; the others ignore it.
      nativeBuildInputs = [ pkgs.pkg-config ];
      extra.buildInputs = [ pkgs.openssl ];
      script = ''
        cd ${dir}
        mkdir -p $out/bin
        cargo build --frozen --release ${args}
        for b in ${toString bins}; do cp target/release/$b $out/bin/; done
      '';
    };

  hostTools = {
    capsule-sign = hostTool { name = "capsule-sign"; dir = "nonos-sign"; bins = [ "capsule-sign" ]; args = "--bin capsule-sign"; };
    stark-enroll = hostTool { name = "stark-enroll"; dir = "nonos-stark-enroll"; bins = [ "nonos-stark-enroll" ]; };
    embed-trailer = hostTool { name = "embed-trailer"; dir = "nonos-bootloader/tools/embed-trailer"; bins = [ "embed-trailer" ]; };
    sign-kernel = hostTool { name = "sign-kernel"; dir = "nonos-bootloader/tools/sign-kernel"; bins = [ "sign-kernel" ]; };
    marketplace-index = hostTool { name = "marketplace-index"; dir = "nonos-mk"; bins = [ "marketplace-index" ]; args = "--bin marketplace-index"; };
    nonos-pack = hostTool { name = "nonos-pack"; dir = "tools/nonos-pack"; bins = [ "nonos-pack" ]; args = "--bin nonos-pack"; };
    nonos-verify = hostTool { name = "nonos-verify"; dir = "nonos-verify"; bins = [ "nonos-verify" ]; };
  };
  # The loader compiles in the kernel's public keys and root, which the seal
  # writes and ek commits. Until the first seal is committed the tree has none,
  # and a build without them leaves the loader out and says so.
  loaderReady = lib.all (f: builtins.pathExists (src.root + "/${f}")) (builtins.attrValues kernelKeys);
in
{
  inherit kernel bootloader hostTools kernelKeys epoch rev loaderReady kernelReady unsealed;
}
