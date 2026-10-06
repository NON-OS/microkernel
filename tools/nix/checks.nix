# `nix flake check`: every proof crate and every static check, each its own
# derivation, so a failure names itself and a pass is cached until its inputs
# change. The build is not green until all of these are.
#
# What stays in its own workflow, because it needs a network or a toolchain no
# lock can pin yet: Kani (cargo kani setup downloads CBMC), Verus, the
# Charon/Aeneas extraction, cargo-fuzz runs and cargo-audit's advisory feed.
{ pkgs, lib, pins, cargo, rustBuild, src, capsules, config, self, starks, busybox }:
let
  # The static and drift checks read the tree as a reviewer does, docs and
  # all; every cargo check reads only its own crate (tools/nix/inputs.py).
  root = src.everything;
  kernelSource = src.crate "." [ "x86_64-nonos.json" ];
  userlandDirs = builtins.attrNames (lib.filterAttrs (_: t: t == "directory") (builtins.readDir (src.root + "/userland")));
  kernelFeatureSets = [ "mldsa3,mlkem768" "mldsa2,mlkem512" "mldsa5,mlkem1024" "dbg-ring,heap-track" "crypto-curve25519" "crypto-ed25519-dalek" ];
  hasLock = d: builtins.pathExists (src.root + "/${d}/Cargo.lock");

  # Every *_proofs crate, the host crates CI tests beside them, and the
  # bootloader's proofs.
  proofDirs =
    map (d: "userland/${d}") (builtins.filter (d: lib.hasSuffix "_proofs" d && hasLock "userland/${d}") userlandDirs)
    ++ [
      "nonos-bootloader/boot_proofs"
      "userland/nonos_secp256k1"
      "userland/nonos_ed25519"
      "userland/nonos_i2cmodel"
      "userland/attest_receipt"
      "nonos-attest-path"
      "nonos-sign"
    ];

  # The live TPM suites run against a software TPM; a skipped live test fails
  # the check rather than passing quietly.
  needsTpm = [ "userland/tpm_enroll_proofs" "userland/tpm_key_proofs" ];
  # certtool is what swtpm_localca signs the EK certificate with.
  tpmTools = [ pins.swtpm pkgs.tpm2-tools pkgs.openssl pkgs.gnutls ];

  # A shell repository of the tree, so the checks that ask git which files are
  # tracked get a real answer instead of an empty list.
  gitTree = ''
    git init -q . && git add -A && git -c user.name=nix -c user.email=nix@localhost commit -q -m tree
  '';

  # Every proof crate passes clippy with -D warnings over all its targets,
  # except these, which are not clean yet. The lists only shrink: a crate
  # leaves when it is fixed, and a new crate joins none of them.
  #   lintLib   clean in the library, not yet in its tests (verify.yml linted
  #             only the library)
  #   lintNone  never linted before the flake, and not clean
  lintLib = [
    "attest_doc_proofs" "capsule_browser_proofs" "crypto_proofs" "fs_proofs" "http_proofs"
    "nym_reply_proofs" "route_proof_proofs" "terminal_line_proofs"
  ];
  lintNone = [ "boot_slots_proofs" "editor_proofs" "input_proofs" ];
  clippy =
    dir:
    let
      name = builtins.baseNameOf dir;
    in
    if builtins.elem name lintNone then ""
    else if builtins.elem name lintLib then "cargo clippy --frozen --release -- -D warnings"
    else "cargo clippy --frozen --release --all-targets -- -D warnings";

  # nonos-sign's `artifacts` test reads the capsule ELFs and the trust set a
  # seal wrote together, so it belongs to the seal, whose verify phase checks
  # the same signatures against the same ELFs; here it would read a tree with
  # no ELFs in it.
  testArgs = dir: lib.optionalString (dir == "nonos-sign") "--lib --bins --test host_trust --test release";

  proof =
    dir:
    let
      tpm = builtins.elem dir needsTpm;
    in
    rustBuild {
      name = "proofs-${builtins.baseNameOf dir}";
      # Its crate, the crates it reaches and the sources it mounts by #[path].
      src = src.crate dir [ ];
      lockFiles = [ (src.root + "/${dir}/Cargo.lock") ];
      nativeBuildInputs = [ pkgs.git ] ++ lib.optionals tpm tpmTools;
      # Overflow checks stay on in the release test build. Capsules ship
      # without them, so an overflow wraps silently on a device; a proof built
      # the same way agrees with the wrapped value and passes. net.ntp's 1968
      # timestamp wrapped the clock that way, with its proof green.
      script = ''
        export RUST_TEST_THREADS=1
        cd ${dir}
        cargo test --frozen --release --config profile.release.overflow-checks=true ${testArgs dir} 2>&1 | tee $TMPDIR/test.log
        ${lib.optionalString tpm ''
          if grep -E "live test skipped|certificate test skipped|live derivation not exercised" $TMPDIR/test.log; then
            echo "a live TPM test was skipped" >&2; exit 1
          fi
        ''}
        ${clippy dir}
        mkdir -p $out && cp $TMPDIR/test.log $out/
      '';
    };

  # The live TPM suites need tpm2-tools, which nixpkgs builds only for Linux,
  # so they run on every Linux machine and in CI, and not on a Mac.
  proofChecks = lib.listToAttrs (map (d: lib.nameValuePair "proofs-${builtins.baseNameOf d}" (proof d))
    (builtins.filter (d: pkgs.stdenv.hostPlatform.isLinux || !(builtins.elem d needsTpm)) proofDirs));

  # A cargo build or test of one crate against its lock, as a check.
  cargoCheck =
    { name, dir, script, buildStd ? false, extra ? [ ], source ? src.crate dir [ ] }:
    rustBuild {
      inherit name buildStd;
      src = source;
      lockFiles = [ (src.root + "/${dir}/Cargo.lock") ];
      nativeBuildInputs = [ pkgs.git ] ++ extra;
      script = ''
        ${script}
        mkdir -p $out
      '';
    };

  # The rest of what verify.yml ran with cargo.
  cargoChecks = {
    # The verification engine's own lint, and its scan of shipping code for
    # panics and stubs.
    nonos-verify = cargoCheck {
      name = "nonos-verify";
      dir = "nonos-verify";
      # hygiene scans the shipping code of the whole tree.
      source = root;
      script = ''
        cargo clippy --frozen --manifest-path nonos-verify/Cargo.toml --all-targets -- -D warnings
        cargo run --frozen --manifest-path nonos-verify/Cargo.toml -- hygiene
      '';
    };
    # The kernel self-attestation proof of concept: embed, footer parse, boot
    # verify and the attacks on each.
    attest-poc = cargoCheck {
      name = "attest-poc";
      dir = "nonos-bootloader/tools/embed-trailer";
      script = "cargo test --frozen --release --manifest-path nonos-bootloader/tools/embed-trailer/Cargo.toml --test kernel_self_attest_poc";
    };
    # The attestation red-team battery, its JSON verdict, and the parser fuzz.
    attest-battery = cargoCheck {
      name = "attest-battery";
      dir = "security/nonos-secops";
      source = src.crate "security/nonos-secops" [ "security/tests" ];
      script = "bash security/tests/run.sh 50000";
    };
    # The browser's DOM prelude, run under node against its own tests.
    qjs-prelude = pkgs.runCommand "qjs-prelude" { src = src.paths [ "userland/nonos_qjs" "userland/capsule_browser/src/browser/http/request.rs" ]; nativeBuildInputs = [ pkgs.nodejs ]; } ''
      cd $src/userland/nonos_qjs && node tests/prelude/run.mjs
      touch $out
    '';
  }
  # Features no profile enables still type-check, one kernel cargo check per
  # set on top of microkernel-core, so the code behind them cannot rot unseen.
  # scripts/check_dark_features.py reads kernelFeatureSets as this lane.
  // lib.listToAttrs (map (set: lib.nameValuePair "kernel-features-${builtins.replaceStrings [ "," ] [ "-" ] set}" (cargoCheck {
    name = "kernel-features-${builtins.replaceStrings [ "," ] [ "-" ] set}";
    dir = ".";
    source = kernelSource;
    buildStd = true;
    extra = [ pins.llvm.clang-unwrapped pins.llvm.llvm ];
    script = ''
      printf 'NONOS-CI-FORK-DEV-SEED-32-BYTE!!' > $TMPDIR/build-placeholder.seed
      export NONOS_SIGNING_KEY=$TMPDIR/build-placeholder.seed
      cargo check --frozen --target x86_64-nonos.json -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem \
        --no-default-features --features microkernel-core,${set}
    '';
  })) kernelFeatureSets);

  # Every profile's kernel type-checks with exactly the features nonos.toml
  # resolves for it, so a profile that names a feature its code cannot build
  # with fails here and not on ek's machine. The capsule ELFs the kernel
  # embeds are empty stand-ins, and so is the trust set of a capsule the seal
  # has not written yet: this checks the code, `nix build` checks the image.
  profileChecks = lib.mapAttrs' (p: _:
    let
      cfg = config.resolve (config.file // { profile = p; });
    in
    lib.nameValuePair "kernel-profile-${p}" (cargoCheck {
      name = "kernel-profile-${p}";
      dir = ".";
      source = kernelSource;
      buildStd = true;
      extra = [ pins.llvm.clang-unwrapped pins.llvm.llvm ];
      script = ''
        printf 'NONOS-CI-FORK-DEV-SEED-32-BYTE!!' > $TMPDIR/build-placeholder.seed
        export NONOS_SIGNING_KEY=$TMPDIR/build-placeholder.seed
        ${lib.concatMapStrings (f: ''
          [ -e "${f}" ] || { mkdir -p "$(dirname "${f}")"; : > "${f}"; }
        '') (lib.concatMap (e: [ "${e.dir}/target/${e.target}/release/${e.bin}" e.cert e.manifest e.trailer ] ++ lib.optional (e.prebuilt != "") e.prebuilt) capsules.catalogue)}
        cargo check --frozen --target x86_64-nonos.json -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem \
          --no-default-features --features ${lib.concatStringsSep "," cfg.features}
      '';
    })) config.profiles;

  python = pins.pythonTools;

  static =
    name: script:
    pkgs.runCommand "static-${name}" {
      src = root;
      nativeBuildInputs = [ python pkgs.git pkgs.jq pkgs.gnugrep pkgs.gawk pkgs.findutils pkgs.diffutils pkgs.bash pkgs.which pins.rust ];
    } ''
      cp -r $src tree && chmod -R u+w tree && cd tree
      ${gitTree}
      ${script}
      touch $out
    '';

  scripts = s: lib.concatMapStrings (c: "python3 ${c}\n") s;

  staticChecks = {
    # verify.yml, hygiene
    static-hygiene = static "hygiene" (scripts [
      "scripts/check_stubs.py --self-test" "scripts/check_stubs.py"
      "scripts/check_allows.py --self-test" "scripts/check_allows.py"
      "scripts/check_unreachable.py --self-test" "scripts/check_unreachable.py"
      "scripts/check_driver_proofs.py"
      "scripts/check_dark_features.py --self-test" "scripts/check_dark_features.py"
    ]);
    # verify.yml, abi-contracts and syscall-reach
    static-abi = static "abi" (scripts [
      "scripts/check_syscall_abi.py" "scripts/check_abi_stable.py"
      "nonos-ci/attack_suite_check.py --self-test" "nonos-ci/boot_attest_check.py --self-test"
      "scripts/check_capsule_ports.py --self-test" "scripts/check_capsule_ports.py"
      "scripts/check_handoff_mirror.py --self-test" "scripts/check_handoff_mirror.py"
      "scripts/check_prebuilt.py" "scripts/check_syscall_args.py" "scripts/check_caps_abi.py"
      "scripts/check_userland_caps.py" "scripts/check_cap_parity.py" "scripts/check_service_caps.py"
      "scripts/check_attest_params.py" "tools/nonos-assumptions"
      "scripts/check_unenforced.py" "scripts/check_unenforced.py --self-test"
      "tools/nonos-linux-coverage --baseline scripts/baselines/linux-syscalls.txt"
      "tools/nonos-wayland-coverage --baseline scripts/baselines/wayland-globals.txt"
      "tools/ratchets/disclosure.py" "tools/nonos-mutant --check"
      "scripts/check_syscall_reach.py --baseline scripts/baselines/syscall-unreachable-count.txt"
    ]);
    # make nonos-mk-static: the grep gates over the whole tree
    static-tree = static "tree" ''
      export HOME=$TMPDIR CARGO_NET_OFFLINE=true
      bash nonos-ci/run-static-checks.sh
    '';
    # verify.yml, evidence-manifest
    static-evidence = static "evidence" ''
      bash verification/evidence/collect-evidence.sh > $TMPDIR/EVIDENCE.json
      diff -u verification/evidence/EVIDENCE.json $TMPDIR/EVIDENCE.json
      jq -e '.proof_systems.lean_specification.sorry_count == 0' $TMPDIR/EVIDENCE.json
      python3 tools/ratchets/proven_functions.py --root .
    '';
  };

  # The flake's own inputs, held to the tree they mirror.
  driftChecks = {
    # capsules.json and store.json are what make prints about the capsules
    # and the store; a Capsule.mk or store entry edited without regenerating
    # them fails here.
    catalogues = pkgs.runCommand "catalogues" {
      src = root;
      nativeBuildInputs = [ pkgs.gnumake python pkgs.findutils pkgs.coreutils pins.rust ];
    } ''
      cp -r $src tree && chmod -R u+w tree && cd tree
      for c in capsule store; do
        make -s --no-print-directory NONOS_IN_FLAKE=1 NONOS_QUIET=1 nonos-mk-$c-catalogue 2>/dev/null > $TMPDIR/$c.json
      done
      python3 tools/nix/catalogues.py --check
      touch $out
    '';
    # inputs.json is what each cargo build reads; a path dependency, #[path]
    # or include added without regenerating it fails here.
    # Forced: the tree tracks a few files .gitignore names, as the flake
    # source holds them, and the table is computed over exactly that source.
    inputs = static "inputs" "git add -A -f . && python3 tools/nix/inputs.py --check";
    # Every git commit a Cargo.lock names has an entry in git-sources.json.
    git-pins = static "git-pins" "python3 tools/nonos-nix-pin --check";
    # The wallpaper catalog's pins are the files the collection packs.
    wallpaper-pins = static "wallpaper-pins" "python3 tools/nonos-wallpaper-pack check";
    # Every manifest follows STARKs main and every lock names the commit
    # flake.lock pins, so STARKs has one pin and it is the flake's.
    starks-pin = static "starks-pin" "python3 tools/nonos-starks-sync --check --rev ${starks.rev}";
    # The wallet vectors Test 5 proves on the machine (userland/
    # capsule_shield_vectors) are STARKs' own, at the commit flake.lock pins.
    shield-vectors-pin = pkgs.runCommand "shield-vectors-pin" { } ''
      for v in transfer-eth withdraw-eth transfer-nox withdraw-nox; do
        for f in request.json seed.json entropy.hex proof.json proof-format7.bin; do
          cmp ${starks}/spec/wallet-vectors-not-before/$v/$f ${src.root + "/userland/capsule_shield_vectors/vectors"}/$v/$f
        done
      done
      touch $out
    '';
    # The committed BusyBox, which make embeds, is the one the flake builds
    # from source. Linux only, as CI checks it; a Mac builds its own anyway.
    busybox-source = pkgs.runCommand "busybox-source" { } (if pkgs.stdenv.hostPlatform.isLinux then ''
      cmp ${busybox}/busybox ${src.root + "/userland/capsule_linux/guests/busybox.elf"}
      touch $out
    '' else "touch $out");
    # The standard library lock -Zbuild-std resolves against, held to the
    # pinned toolchain's.
    rust-src-lock = pkgs.runCommand "rust-src-lock" { } ''
      cmp ${cargo.rustSrcLock} $(readlink -f ${pins.rust}/lib/rustlib/src)/rust/library/Cargo.lock
      touch $out
    '';
    # nonos.toml resolves, for every profile, against Cargo.toml's features.
    config = pkgs.writeText "config" (builtins.toJSON (map (p: (config.resolve (config.file // { profile = p; })).features) (builtins.attrNames config.profiles)));
  };
in
proofChecks // cargoChecks // profileChecks // staticChecks // driftChecks
