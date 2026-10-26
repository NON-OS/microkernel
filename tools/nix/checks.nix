# `nix flake check`: every proof crate and every static check, each its own
# derivation, so a failure names itself and a pass is cached until its inputs
# change. The build is not green until all of these are.
#
# What stays in its own workflow, because it needs a network or a toolchain no
# lock can pin yet: Kani (cargo kani setup downloads CBMC), Verus, the
# Charon/Aeneas extraction, cargo-fuzz runs and cargo-audit's advisory feed.
{ pkgs, lib, pins, cargo, rustBuild, src, capsules, config, image, self, starks, busybox, periodic }:
let
  # The static and drift checks read the tree as a reviewer does, docs and
  # all; every cargo check reads only its own crate (tools/nix/inputs.py).
  root = src.everything;
  kernelSource = src.crate "." [ "x86_64-nonos.json" ];
  userlandDirs = builtins.attrNames (lib.filterAttrs (_: t: t == "directory") (builtins.readDir (src.root + "/userland")));
  kernelFeatureSets = [ "mldsa3,mlkem768" "mldsa2,mlkem512" "mldsa5,mlkem1024" "dbg-ring,heap-track" "crypto-curve25519" "crypto-ed25519-dalek" "nonos-dev-unconfined-dma,nonos-dev-unremapped-interrupts" ];
  hasLock = d: builtins.pathExists (src.root + "/${d}/Cargo.lock");

  # Every *_proofs crate, and every crate whose manifest asks for the lane
  # ([package.metadata.nonos] test-lane = true), each mapped to its test-args.
  # scripts/check_test_lanes.py writes test-lanes.json from the manifests.
  lanes = lib.importJSON ./test-lanes.json;
  proofDirs =
    map (d: "userland/${d}") (builtins.filter (d: lib.hasSuffix "_proofs" d && hasLock "userland/${d}") userlandDirs)
    ++ builtins.attrNames lanes;

  # The live TPM suites run against a software TPM; a skipped live test fails
  # the check rather than passing quietly.
  needsTpm = [ "userland/tpm_enroll_proofs" "userland/tpm_key_proofs" ];
  # certtool is what swtpm_localca signs the EK certificate with.
  tpmTools = [ pins.swtpm pkgs.tpm2-tools pkgs.openssl pkgs.gnutls ];

  # The proof crates handed the periodic cache nonos.shield ships, by
  # NONOS_PERIODIC_CACHE, as capsules.nix hands it to the capsule. A test that
  # finds it missing says so, and the check fails on that line.
  needsPeriodic = [ "userland/shield_vectors_proofs" ];

  # A shell repository of the tree, so the checks that ask git which files are
  # tracked get a real answer instead of an empty list.
  gitTree = ''
    # git permits a macro attribute ([attr] ...) only in a top-level
    # .gitattributes; one in a subdirectory is rejected -- a warning in older
    # git, a hard error from git 2.55 on. The vendored pqclean tree ships such
    # a macro (third_party/pqclean/.gitattributes), so neutralize any [attr]
    # line in a third_party/ .gitattributes in this throwaway working copy,
    # or the git add below fails. The committed files and the pqclean
    # tree-hash pin (read from HEAD) are untouched, and third_party is already
    # out of scope for our whitespace gates.
    find third_party -name .gitattributes -type f \
      -exec sed -i 's/^\[attr\]/# [attr]/' {} + 2>/dev/null || true
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
    "nym_reply_proofs" "route_proof_proofs"
  ];
  lintNone = [ "boot_slots_proofs" "editor_proofs" "input_proofs" ];
  clippy =
    dir: features:
    let
      name = builtins.baseNameOf dir;
    in
    if builtins.elem name lintNone then ""
    else if builtins.elem name lintLib then "cargo clippy --frozen --release ${features} -- -D warnings"
    else "cargo clippy --frozen --release --all-targets ${features} -- -D warnings";

  # What a marked crate's manifest adds to cargo test, as nonos-sign's does.
  testArgs = key: lanes.${key} or "";

  # A feature lane is <dir>+<feature> (feature-lanes in the manifest): the
  # crate's tests and its lint again with that feature on.
  laneParts = key: lib.splitString "+" key;
  laneName = key: "proofs-${builtins.replaceStrings [ "+" ] [ "-" ] (builtins.baseNameOf key)}";

  proof =
    key:
    let
      dir = builtins.head (laneParts key);
      features = lib.concatMapStrings (f: "--features ${f}") (builtins.tail (laneParts key));
      tpm = builtins.elem dir needsTpm;
      cache = builtins.elem dir needsPeriodic;
    in
    rustBuild {
      name = laneName key;
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
        ${lib.optionalString cache "export NONOS_PERIODIC_CACHE=${periodic}/periodic.top"}
        cd ${dir}
        cargo test --frozen --release --config profile.release.overflow-checks=true ${testArgs key} 2>&1 | tee $TMPDIR/test.log
        ${lib.optionalString tpm ''
          if grep -E "live test skipped|certificate test skipped|live derivation not exercised" $TMPDIR/test.log; then
            echo "a live TPM test was skipped" >&2; exit 1
          fi
        ''}
        ${lib.optionalString cache ''
          if grep "real cache test skipped" $TMPDIR/test.log; then
            echo "a real periodic cache test was skipped" >&2; exit 1
          fi
        ''}
        ${clippy dir features}
        mkdir -p $out && cp $TMPDIR/test.log $out/
      '';
    };

  # The live TPM suites need tpm2-tools, which nixpkgs builds only for Linux,
  # so they run on every Linux machine and in CI, and not on a Mac.
  proofChecks = lib.listToAttrs (map (d: lib.nameValuePair (laneName d) (proof d))
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
  # The files each stand-in is made at, one a line. A file, not the script:
  # written inline, one command per file, the script outgrew the 128 KiB a
  # single environment string may hold, and every profile check failed to
  # start ("Argument list too long").
  standIns = pkgs.writeText "kernel-profile-stand-ins" (lib.concatMapStrings (f: f + "\n")
    (lib.concatMap (e: [ "${e.dir}/target/${e.target}/release/${e.bin}" e.cert e.manifest e.trailer ]
      ++ lib.optional (e.prebuilt != "") e.prebuilt) capsules.catalogue));

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
        while IFS= read -r f; do
          [ -e "$f" ] || { mkdir -p "$(dirname "$f")"; : > "$f"; }
        done < ${standIns}
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
      "scripts/check_test_lanes.py --self-test" "scripts/check_test_lanes.py"
      "-m unittest discover -s tools/nonos_selftest -p 'test_*.py'"
      "-m unittest discover -s tools/agentproof_gauntlet -t tools -p 'test_*.py'"
    ]);
    # verify.yml, abi-contracts and syscall-reach
    static-abi = static "abi" (scripts [
      "scripts/check_syscall_abi.py --self-test" "scripts/check_syscall_abi.py" "scripts/check_abi_stable.py"
      "nonos-ci/attack_suite_check.py --self-test" "nonos-ci/boot_attest_check.py --self-test"
      "scripts/check_capsule_ports.py --self-test" "scripts/check_capsule_ports.py"
      "scripts/check_handoff_mirror.py --self-test" "scripts/check_handoff_mirror.py"
      "scripts/check_blk_wire.py --self-test" "scripts/check_blk_wire.py"
      "scripts/check_prebuilt.py" "scripts/check_syscall_args.py" "scripts/check_caps_abi.py"
      "scripts/check_userland_caps.py" "scripts/check_cap_parity.py" "scripts/check_service_caps.py"
      "scripts/check_baked_names.py --self-test" "scripts/check_baked_names.py"
      "scripts/check_copy_only_features.py --self-test" "scripts/check_copy_only_features.py"
      "tools/nonos-cap-audit --self-test" "tools/nonos-cap-audit"
      "tools/nonos-pcap-egress --self-test" "tools/nonos_egress/mutants.py" "tools/nonos-efi-vars --self-test" "scripts/check_sealed_caps.py --self-test"
      "scripts/check_kernel_wait.py --self-test" "scripts/check_kernel_wait.py"
      "scripts/check_redacted_addresses.py --self-test" "scripts/check_redacted_addresses.py"
      "scripts/check_attest_params.py" "tools/nonos-assumptions"
      "scripts/check_unenforced.py" "scripts/check_unenforced.py --self-test"
      "tools/ratchets/linux_calls.py --self-test"
      "tools/nonos-linux-coverage --baseline scripts/baselines/linux-syscalls.txt"
      "tools/nonos-wayland-coverage --baseline scripts/baselines/wayland-globals.txt"
      "tools/ratchets/disclosure.py" "tools/nonos-mutant --check" "nonos-ci/check-ui-copy.py --self-test" "nonos-ci/check-ui-copy.py"
      "tools/nonos-ui-measure --self-test" "tools/nonos-unit-check --self-test"
      "scripts/check_syscall_reach.py --baseline scripts/baselines/syscall-unreachable-count.txt"
    ]);
    # Every capsule a release profile ships has its sealed trust set, but for
    # those scripts/baselines/unsealed.txt names, which may only shrink.
    static-release-trust =
      let
        resolved = map (p: config.resolve (config.file // { profile = p; })) (builtins.attrNames config.profiles);
        slugs = lib.unique (lib.concatMap (c: map (e: e.slug) (image.shipped c.enabled)) (builtins.filter (c: c.release) resolved));
      in
      static "release-trust" (scripts [
        "scripts/check_release_trust.py --self-test"
        "scripts/check_release_trust.py --baseline scripts/baselines/unsealed.txt --doc docs/build/feature-inventory.md ${toString slugs}"
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
      for c in capsule capsule-dev store; do
        make -s --no-print-directory NONOS_IN_FLAKE=1 NONOS_QUIET=1 nonos-mk-$c-catalogue 2>/dev/null > $TMPDIR/$c.json
      done
      python3 tools/nix/catalogues.py --check
      touch $out
    '';
    # Every capsule make builds is built for the user target asked of it
    # (nonos-ci/check-capsule-targets.py): until C-23 the rule built x86_64
    # whatever the aarch64 lanes asked. make -n only, so nothing is built.
    capsule-targets = pkgs.runCommand "capsule-targets" {
      src = root;
      nativeBuildInputs = [ pkgs.gnumake python pkgs.findutils pkgs.coreutils pins.rust ];
    } ''
      cp -r $src tree && chmod -R u+w tree && cd tree
      export HOME=$TMPDIR
      python3 nonos-ci/check-capsule-targets.py
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
    # nonos-shield-prove, the prover nonos.shield proves with, reproduces
    # those four vectors byte for byte and verifies them, at the STARKs
    # commit flake.lock pins. Four proofs of minutes each, so its own check,
    # not the proof lane.
    shield-prove-vectors = cargoCheck {
      name = "shield-prove-vectors";
      dir = "nonos-shield-prove";
      script = ''
        export STARKS_SPEC=${starks}/spec
        cd nonos-shield-prove
        cargo test --frozen --release --features parallel -- --include-ignored
      '';
    };
    # Test 5 on the host: shield_vectors_proofs' binary proves the four wallet
    # vectors with capsule_shield_vectors' own modules, transfer-eth from
    # nothing (the tree it builds held to the shipped cache) and the other
    # three from the shipped cache, each held to the pinned bytes. About four
    # minutes, so its own check: the proof lane's cargo test never runs the
    # binary.
    shield-vectors-prove = cargoCheck {
      name = "shield-vectors-prove";
      dir = "userland/shield_vectors_proofs";
      script = ''
        export NONOS_PERIODIC_CACHE=${periodic}/periodic.top
        cd userland/shield_vectors_proofs
        cargo run --frozen --release
      '';
    };
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
    # nonos.toml resolves, for every profile, against Cargo.toml's features;
    # a release image that asks for the meter fails to evaluate, while its
    # development twin, the same request with dev = true, resolves; and every
    # development twin carries the meter, and settle unless the profile takes
    # the network out.
    config =
      let
        names = builtins.attrNames config.profiles;
        meter = p: dev: { profile = p; inherit dev; features = [ "nonos-capsule-payment" ]; };
        resolves = raw: (builtins.tryEval (builtins.deepSeq (config.resolve raw).features true)).success;
        release = builtins.filter (p: config.profiles.${p}.loader != "dev-qemu") names;
        carries = p: lib.all (f: builtins.elem f (config.resolve { profile = p; dev = true; }).enabled
          || builtins.elem f config.profiles.${p}.drop) [ "nonos-capsule-payment" "nonos-capsule-nox-settle" ];
      in
      assert lib.assertMsg (release != [ ] && !(lib.any (p: resolves (meter p false)) release))
        "config: a release profile resolved with nonos-capsule-payment, the meter";
      assert lib.assertMsg (lib.all (p: resolves (meter p true)) names)
        "config: a development twin refused nonos-capsule-payment, the meter";
      assert lib.assertMsg (lib.all carries names)
        "config: a development twin left out the meter or settle";
      pkgs.writeText "config" (builtins.toJSON (map (p: (config.resolve (config.file // { profile = p; })).features) (builtins.attrNames config.profiles)));
    # Capsule serial output: a profile's development twin keeps it, and no
    # profile a release loader boots may carry it. The last assertion is the
    # rule's self-test: qemu, which keeps it, refused on a standard loader.
    profile-serial =
      let
        has = r: builtins.elem "capsule-serial-debug" r.enabled;
        res = p: dev: config.resolve (config.file // { profile = p; inherit dev; });
        onStandard = config.resolve (config.file // { profile = "qemu"; dev = false; loader = "standard"; });
      in
      assert has (res "standard" true) && has (res "hardened" true);
      assert !(has (res "standard" false)) && !(has (res "hardened" false)) && !(has (res "airgapped" false));
      assert has (res "core" true) == has (res "core" false);
      assert !(builtins.tryEval (has onStandard)).success;
      pkgs.writeText "profile-serial" "a development twin keeps capsule serial output; a release loader never";
  };

  # The determinism property, proved on Linux alone. A capsule is built twice in
  # the one sandbox from two different absolute roots (so the absolute path of
  # its ../libc path dependency differs, the R2 leak) and under two different
  # __CARGO_DEFAULT_LIB_METADATA values (the exact channel cargo folds the rustc
  # host triple through, so a stand-in foreign host, the R1 leak), then the two
  # ELFs are compared. With the nonos-rustc wrapper recomputing each crate's
  # -C metadata from host- and path-neutral inputs, the two are byte-identical;
  # revert the wrapper (or set NONOS_RUSTC_NORMALIZE=0) and they differ, which
  # is how this check earns its keep.
  reproChecks =
    let
      dir = "userland/capsule_proof_io";
      capsuleSrc = src.crate dir [ "userland/x86_64-nonos-user.json" ];
      vendor = cargo.vendor {
        name = "repro-proof-io";
        lockFiles = [ (src.root + "/${dir}/Cargo.lock") ];
        buildStd = true;
      };
    in
    {
      repro-host-independence = pkgs.stdenvNoCC.mkDerivation {
        name = "repro-host-independence";
        src = capsuleSrc;
        nativeBuildInputs = [ pins.rust pins.python pins.llvm.clang-unwrapped pins.llvm.llvm pkgs.git ];
        dontConfigure = true;
        dontFixup = true;
        dontInstall = true;
        buildPhase = ''
          runHook preBuild
          export AR=llvm-ar RANLIB=llvm-ranlib
          # The wrapper's own property test, as a gate.
          python3 ${./nonos-rustc.py} --self-test
          ${cargo.setup vendor}

          # Build proof_io once in a given root, under a given stand-in host.
          buildone() {
            root="$1"; fakehost="$2"; dest="$3"
            rm -rf "$root"; mkdir -p "$root"
            cp -r ${capsuleSrc}/. "$root"/
            chmod -R u+w "$root"
            export NONOS_WORK="$root"
            export NONOS_RUSTC_IDS="$root/.nonos-rustc-ids"; mkdir -p "$NONOS_RUSTC_IDS"
            export CARGO_TARGET_DIR="$root/target"
            export __CARGO_DEFAULT_LIB_METADATA="$fakehost"
            ( cd "$root/${dir}" &&
              cargo build --frozen --release --target ../x86_64-nonos-user.json -Zbuild-std=core )
            cp "$CARGO_TARGET_DIR/x86_64-nonos-user/release/proof_io" "$dest"
          }

          # Two roots of different names and lengths, two foreign host identities.
          buildone "$NIX_BUILD_TOP/repro-root-alpha-longer-path" \
            "nonos-stand-in-host-alpha-x86_64-unknown-linux-gnu" "$TMPDIR/a.elf"
          buildone "$NIX_BUILD_TOP/r2" \
            "nonos-stand-in-host-bravo-aarch64-apple-darwin" "$TMPDIR/b.elf"

          echo "a: $(sha256sum < $TMPDIR/a.elf)"
          echo "b: $(sha256sum < $TMPDIR/b.elf)"
          if ! cmp "$TMPDIR/a.elf" "$TMPDIR/b.elf"; then
            echo "repro-host-independence: the two builds are NOT byte-identical" >&2
            exit 1
          fi
          mkdir -p $out
          cp "$TMPDIR/a.elf" $out/proof_io
          runHook postBuild
        '';
      };
    };

  # No release capsule carries the local test chain's names (P5).
  releaseChecks.release-names = import ./release_names.nix { inherit pkgs lib capsules; };
in
proofChecks // cargoChecks // profileChecks // staticChecks // driftChecks // reproChecks // releaseChecks
