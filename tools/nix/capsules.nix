# Every capsule, one derivation each, from the catalogue make itself prints
# (capsules.json, kept in step by the capsule-catalogue check).
#
# A capsule builds exactly as nonos-mk/capsule.mk builds it: cargo build
# --release for x86_64-nonos-user with -Zbuild-std, from the capsule's own
# directory and its own lock. The prebuilt capsules (the crates.io tools and
# the Linux userland) take their binary from the rule that makes it.
{ pkgs, lib, pins, rustBuild, src, periodic, linuxUserland, busybox, packageMirror ? "" }:
let
  catalogue = lib.importJSON ./capsules.json;
  userTarget = "x86_64-nonos-user";
  # Every capsule's own crates, and the target it is built for.
  targetJson = "userland/${userTarget}.json";
  source = dir: src.crate dir [ targetJson ];

  # The standard library with NONOS's platform layer applied, which the std
  # capsules (ripgrep, sd and the other crates.io tools) are built against.
  # toolchain/nonos-std/apply.sh patches a rustup sysroot in place; here it
  # patches a copy, so the pinned toolchain itself is never touched.
  mkRustStd = base: pkgs.runCommand "rust-nonos-std" { nativeBuildInputs = [ pins.python ]; } ''
    cp -a ${base} $out
    chmod -R u+w $out
    src=$(readlink -f ${base}/lib/rustlib/src)
    rm -rf $out/lib/rustlib/src
    cp -rL $src $out/lib/rustlib/src
    chmod -R u+w $out/lib/rustlib/src
    mkdir -p $TMPDIR/shim
    printf '#!/bin/sh\necho %s\n' "$out" > $TMPDIR/shim/rustup-sysroot
    cat > $TMPDIR/shim/rustup <<EOF
    #!/bin/sh
    exec $TMPDIR/shim/rustup-sysroot
    EOF
    chmod +x $TMPDIR/shim/*
    cp -r ${src.paths [ "toolchain/nonos-std" ]}/toolchain/nonos-std $TMPDIR/nonos-std
    chmod -R u+w $TMPDIR/nonos-std
    sed -i 's#^PY_BIN=/usr/bin/python3#PY_BIN=python3#' $TMPDIR/nonos-std/apply.sh
    PATH=$TMPDIR/shim:$PATH bash $TMPDIR/nonos-std/apply.sh
    # rustc finds its sysroot from the librustc_driver it loads, which is the
    # pinned toolchain's, so the copy says where its own sysroot is, unless
    # the caller (cargo's -Zbuild-std) already names one.
    mv $out/bin/rustc $out/bin/.rustc-unwrapped
    cat > $out/bin/rustc <<EOF
    #!${pkgs.runtimeShell}
    for a in "\$@"; do
      case "\$a" in --sysroot|--sysroot=*) exec $out/bin/.rustc-unwrapped "\$@" ;; esac
    done
    exec $out/bin/.rustc-unwrapped --sysroot $out "\$@"
    EOF
    chmod +x $out/bin/rustc
    [ "$($out/bin/rustc --print sysroot)" = "$out" ]
  '';
  rustStd = mkRustStd pins.rust;

  # A crate that compiles C for the capsule triple (blake3's SIMD, in the
  # crypto and shield capsules) gets it from cc-rs, which takes the build
  # machine's compiler. An aarch64 Linux gcc cannot target x86_64, so there
  # the capsule triple's C compiler is an x86_64 cross gcc. x86_64 Linux and
  # macOS build as they always have, so nothing changes there.
  nativeX86 = !(pkgs.stdenv.hostPlatform.isLinux && pkgs.stdenv.hostPlatform.isAarch64);
  x86Cc = pkgs.pkgsCross.gnu64.stdenv.cc;

  build =
    args:
    rustBuild ({
      buildStd = true;
      cc = false;
      nativeBuildInputs = [ pins.llvm.clang-unwrapped pins.llvm.llvm ]
        ++ lib.optional (!nativeX86) x86Cc;
      extra = {
        AR = "llvm-ar";
        RANLIB = "llvm-ranlib";
      } // lib.optionalAttrs (!nativeX86) {
        CC_x86_64_nonos_user = "${x86Cc}/bin/x86_64-unknown-linux-gnu-gcc";
      };
    } // args);

  # nonos_rt.o, the start object every std capsule links first.
  rt = build {
    name = "nonos-rt";
    src = source "toolchain/nonos-rt";
    lockFiles = [ (src.root + "/toolchain/nonos-rt/Cargo.lock") ];
    script = ''
      mkdir -p $out
      cd toolchain/nonos-rt
      cargo rustc --frozen --release --target ../../userland/${userTarget}.json \
        -Zbuild-std=core -- --emit obj=$out/nonos_rt.o
    '';
  };

  # The market index and the model catalogue are signed by the market
  # operator, so the capsules embed the signed files ek commits under
  # nonos-data/. Without them the capsules embed nothing, as make does without
  # the operator seed.
  signedInputs = ''
    mkdir -p target/market target/models
    if [ -f nonos-data/market/index.bin ]; then cp nonos-data/market/index.bin target/market/index.bin
    else : > target/market/index.bin; fi
    if [ -f nonos-data/models/catalogue.bin ]; then cp nonos-data/models/catalogue.bin target/models/catalogue.bin
    else : > target/models/catalogue.bin; fi
    # The commit the trust set was enrolled at, as a reuse build pins it, so
    # About and Settings name the same build from one enrollment to the next.
    # Only a capsule that reads NONOS_BUILD_SHA is given the file
    # (tools/nix/inputs.py, MARKERS), so a seal rebuilds only those.
    if [ -f nonos-data/trust/COMMIT ]; then export NONOS_BUILD_SHA=$(cat nonos-data/trust/COMMIT); fi
  '';

  compiled =
    e:
    build {
      name = "nonos-capsule-${e.slug}";
      src = source e.dir;
      lockFiles = [ (src.root + "/${e.dir}/Cargo.lock") ];
      rust = if e.needs_rt then rustStd else pins.rust;
      script = ''
        # The capsule C (QuickJS, minimp3) asks for "clang" by name. On macOS
        # the first clang is the stdenv wrapper, which adds host flags, so the
        # same C compiled to different code there than on Linux and the
        # capsule was not reproducible across hosts. Put the unwrapped one
        # first, as the bootloader build does (image.nix).
        export PATH=${pins.llvm.clang-unwrapped}/bin:$PATH
        ${signedInputs}
        ${lib.optionalString (e.slug == "linux") ''
          # The personality's built-in BusyBox, built from source here; the
          # committed guests/busybox.elf is the same bytes, for make.
          cp ${busybox}/busybox ${e.dir}/guests/busybox.elf${lib.optionalString (packageMirror != "") ''

          # The NONOS package mirror in-tree tools install from (nonos.toml).
          export NONOS_PACKAGE_MIRROR=${packageMirror}''}
        ''}
        ${lib.optionalString (e.slug == "shield" || e.slug == "shield-vectors") ''
          # The periodic cache the prover proves from, embedded in the capsule.
          export NONOS_PERIODIC_CACHE=${periodic}/periodic.top
        ''}
        cd ${e.dir}
        # Pin curve25519-dalek's backend. Left to choose, it reads the BUILD
        # host's CPU and takes an AVX/SIMD backend on an x86 host and the serial
        # one on aarch64, so the same capsule came out with different bytes on a
        # Linux runner than on a macOS one and `reproducible / compare` failed.
        # The kernel and image builds already pin serial (mk/20-build.mk,
        # image.nix); the capsule build must match so the artifact is the same
        # on any host. Inert for capsules that do not pull curve25519.
        export RUSTFLAGS='--cfg curve25519_dalek_backend="serial"${lib.optionalString e.needs_rt " -Clink-arg=${rt}/nonos_rt.o"}'
        cargo build --frozen --release --target ../${userTarget}.json \
          ${lib.optionalString (e.cargo_features != "") "--features ${e.cargo_features}"} \
          -Zbuild-std=${e.build_std} \
          ${lib.optionalString (e.build_std_features != "") "-Zbuild-std-features=${e.build_std_features}"}
        mkdir -p $out
        cp target/${e.target}/release/${e.bin} $out/${e.bin}
      '';
    };

  # The crates.io tools, unmodified, built through the std platform layer from
  # the source vendored under userland/upstream-src (ripgrep from its crate
  # and the lock it was published with). The make rules install these with
  # `cargo install`; --locked here holds every dependency to the lock.
  toolFeatures = {
    grex = "--no-default-features --features cli";
    tokei = "--no-default-features --features cli";
  };
  rdrandTools = [ "grex" "dotenv-linter" "pastel" "jsonxf" "tokei" "huniq" "csview" ];

  ripgrepCrate = pkgs.fetchurl {
    name = "ripgrep-14.1.1.crate";
    url = "https://static.crates.io/crates/ripgrep/ripgrep-14.1.1.crate";
    sha256 = "f77b8032dc584527975f34aa5a897d0ef5a785573fda778771a614ff9da501d9";
  };

  upstreamTool =
    { tool, dir, lock, features ? "", rdrand ? false }:
    build {
      name = "nonos-upstream-${tool}";
      # ripgrep's crate is unpacked in the build; the others are in the tree.
      src = if tool == "rg" then src.paths [ ".cargo" targetJson ] else source dir;
      lockFiles = [ lock ];
      rust = rustStd;
      script = ''
        ${lib.optionalString (tool == "rg") ''
          mkdir -p ${dir}
          tar xzf ${ripgrepCrate} -C ${dir} --strip-components=1
          cmp ${dir}/Cargo.lock ${lock}
        ''}
        export RUSTFLAGS='-Clink-arg=${rt}/nonos_rt.o${lib.optionalString rdrand " --cfg getrandom_backend=\"rdrand\""}'
        # One codegen unit, as make builds them (mk/20-build.mk): fat LTO over
        # several units merges them in the order the threads finish.
        export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
        cd ${dir}
        cargo install --frozen --path . ${features} \
          --target $NIX_BUILD_TOP/source/userland/${userTarget}.json \
          -Zbuild-std=std,panic_abort -Zbuild-std-features=compiler-builtins-mem \
          --root $out --no-track --bin ${tool}
      '';
    };

  upstream = {
    ripgrep = upstreamTool { tool = "rg"; dir = "userland/upstream-src/ripgrep-14.1.1"; lock = ./locks/ripgrep-14.1.1.Cargo.lock; };
    sd = upstreamTool { tool = "sd"; dir = "userland/upstream-src/sd-1.0.0"; lock = src.root + "/userland/upstream-src/sd-1.0.0/Cargo.lock"; };
    tokio-smoke = upstreamTool { tool = "tokio-smoke"; dir = "userland/upstream-src/tokio-smoke"; lock = src.root + "/userland/upstream-src/tokio-smoke/Cargo.lock"; };
  } // lib.genAttrs rdrandTools (t: upstreamTool {
    tool = t;
    dir = "userland/upstream-src/${t}";
    lock = src.root + "/userland/upstream-src/${t}/Cargo.lock";
    features = toolFeatures.${t} or "--no-default-features";
    rdrand = true;
  });

  # A prebuilt capsule names the file make would copy; this maps that file to
  # the derivation that makes it.
  prebuiltFrom =
    e:
    let
      p = e.prebuilt;
      m = builtins.match "target/upstream-([^/]+)/.*" p;
      l = builtins.match "target/linux-userland/(.*)" p;
    in
    if m != null then "${upstream.${builtins.head m}}/bin/${baseNameOf p}"
    else if l != null then "${linuxUserland}/${builtins.head l}"
    else throw "capsule ${e.slug}: no derivation makes ${p}";

  prebuilt =
    e:
    pkgs.runCommand "nonos-capsule-${e.slug}" { } ''
      mkdir -p $out
      cp ${prebuiltFrom e} $out/${e.bin}
    '';

  one = e: if e.prebuilt != "" then prebuilt e else compiled e;
in
{
  inherit catalogue rustStd mkRustStd rt upstream;
  bySlug = lib.listToAttrs (map (e: lib.nameValuePair e.slug (one e)) catalogue);
}
