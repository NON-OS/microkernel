# The Linux programs every image carries in its store, built by
# tools/nonos-linux-userland-build exactly as make builds them, from the
# sources tools/nix/sources.txt pins. Each pin is a fixed output derivation
# keyed by its sha256, fetched once and cached, and the script finds them all
# in its source cache, so it never reaches the network.
{ pkgs, lib, pins, src, cargo }:
let
  lines = builtins.filter (l: l != "" && !(lib.hasPrefix "#" l)) (lib.splitString "\n" (builtins.readFile ./sources.txt));
  parse = l: let f = builtins.filter (x: x != "") (lib.splitString " " l); in {
    name = builtins.elemAt f 0;
    url = builtins.elemAt f 1;
    hash = builtins.elemAt f 2;
  };
  sources = lib.listToAttrs (map (l: let p = parse l; in lib.nameValuePair p.name p) lines);

  # ftp.gnu.org resets connections from CI runners often enough to fail a
  # build, so a GNU pin falls back to the kernel.org GNU mirror. The sha256
  # decides what is accepted, so the bytes are the pin's whichever answers.
  gnu = "https://ftp.gnu.org/gnu/";
  urlsOf = url: [ url ] ++ lib.optional (lib.hasPrefix gnu url) ("https://mirrors.kernel.org/gnu/" + lib.removePrefix gnu url);
  fetchPin = p: pkgs.fetchurl { urls = urlsOf p.url; sha256 = p.hash; };

  tarballs = lib.filterAttrs (n: p: !(lib.hasPrefix "zig-" n || lib.hasPrefix "cmake-" n || lib.hasPrefix "git+" p.url)) sources;

  # The release zig from ziglang.org, by the sha256 in sources.txt, so every
  # host cross-compiles with the same compiler bytes.
  zigPlatform = {
    x86_64-linux = "x86_64-linux";
    aarch64-linux = "aarch64-linux";
    aarch64-darwin = "aarch64-macos";
  }.${pkgs.stdenv.hostPlatform.system};
  zig = pkgs.runCommand "zig-0.16.0" { } ''
    mkdir -p $out
    tar -xJf ${fetchPin sources."zig-${zigPlatform}"} -C $out --strip-components=1
  '';

  # A source pinned by commit: git+REPOSITORY@COMMIT, held to the tree hash
  # sources.txt gives it.
  gitTree =
    name:
    let
      m = builtins.match "git\\+(.*)@([0-9a-f]+)" sources.${name}.url;
    in
    pkgs.fetchgit {
      url = builtins.head m;
      rev = builtins.elemAt m 1;
      hash = sources.${name}.hash;
      fetchSubmodules = false;
    };
  llama = gitTree "llama.cpp";

  # What tools/linux-userland/lib/terminal.sh fetches.
  terminalPins = [ "ncurses" "readline" ] ++ map (n: "readline83-00${toString n}") [ 1 2 3 4 5 6 ];

  # What each program's recipe (tools/linux-userland/NAME.sh) reads beyond
  # itself: the pins it fetches, the files of the tree it names, and the
  # sources pinned by commit it takes through tools/linux-userland/lib/git.sh,
  # which get here as NONOS_SRC_<name>. A build sees these and nothing else,
  # so adding a program or a pin, or editing one program, rebuilds only that
  # program.
  # A crates.io program (tools/linux-userland/lib/crate.sh): its crate, and
  # every dependency vendored from the lock it was published with, which
  # tools/nix/locks keeps (tools/nix/vendor.nix).
  crate = name: pin: lock: {
    pins = [ pin ];
    files = [ "tools/linux-userland/lib/crate.sh" "tools/nix/locks/${baseNameOf lock}" ];
    nativeBuildInputs = [ pins.rustMusl ];
    env = cargo.setup (cargo.vendor { name = "linux-${name}"; lockFiles = [ lock ]; });
  };

  recipes = {
    lua.pins = [ "lua" ];
    zstd.pins = [ "zstd" ];
    john.pins = [ "john" ];
    python = {
      pins = [ "python" "zlib" "openssl" "util-linux" "libffi" "xz" "sqlite" "bzip2" ] ++ terminalPins;
      files = [ "tools/linux-userland/lib/terminal.sh" "tools/nonos-python-stdlib-zip" "nonos-data/cacert.pem" ];
      # The Python the cross build runs on the build machine compresses the
      # standard library zip, so it needs zlib; it reaches only that host
      # compiler, never zig.
      buildInputs = [ pkgs.zlib ];
    };
    sqlite3 = {
      pins = [ "sqlite" ] ++ terminalPins;
      files = [ "tools/linux-userland/lib/terminal.sh" ];
    };
    qjs.pins = [ "quickjs" ];
    jq.pins = [ "jq" ];
    tclsh.pins = [ "tcl" ];
    mruby = {
      pins = [ "mruby" ];
      git = [ "mruby" ];
      files = [ "tools/linux-userland/lib/git.sh" ];
      # minirake runs the rake nixpkgs ships with Ruby.
      nativeBuildInputs = [ pkgs.ruby ];
    };
    rg = crate "rg" "ripgrep" ./locks/ripgrep-15.2.0.Cargo.lock;
    fd = crate "fd" "fd-find" ./locks/fd-find-10.5.0.Cargo.lock;
    gojq = {
      pins = [ "gojq" "go-yaml" "timefmt-go" "go-isatty" "go-runewidth" "stringish" "uax29" "x-sys" "go-cmp.mod" "x-sys-0.6.mod" ];
      nativeBuildInputs = [ pins.go pkgs.unzip ];
    };
    perl = {
      pins = [ "perl" "perl-cross" ];
      files = [ "userland/linux_userland/perl-lib.txt" ];
      # LLVM's nm, objdump and readelf read x86-64 ELF on any build machine.
      nativeBuildInputs = [ pkgs.llvmPackages.bintools-unwrapped ];
    };
    nano = {
      pins = [ "nano" "ncurses" ];
      files = [ "tools/linux-userland/lib/terminal.sh" ];
    };
    make.pins = [ "make" ];
    openssl.pins = [ "openssl" ];
    qwenchat = {
      pins = [ "llama.cpp" ];
      files = [ "tools/nonos-cmake" "userland/linux_guests/cpp" ];
      env = "export NONOS_LLAMA_SRC=${llama}";
    };
  };

  program =
    name:
    let
      r = { files = [ ]; git = [ ]; buildInputs = [ ]; nativeBuildInputs = [ ]; env = ""; } // recipes.${name};
      pinned = map (n: sources.${n}) r.pins;
      # The lines of sources.txt this program fetches, as the script reads them.
      pinLines = pkgs.writeText "sources-${name}.txt" (lib.concatMapStrings (p: "${p.name} ${p.url} ${p.hash}\n") pinned);
      cache = pkgs.linkFarm "nonos-src-cache-${name}" (map (p: {
        name = baseNameOf p.url;
        path = fetchPin p;
      }) (builtins.filter (p: tarballs ? ${p.name}) pinned));
    in
    pkgs.stdenv.mkDerivation {
      name = "nonos-linux-${name}";
      src = src.userlandScripts name r.files;
      nativeBuildInputs = [ pins.cmake pkgs.perl pkgs.which pkgs.gnumake pkgs.pkg-config ] ++ r.nativeBuildInputs;
      inherit (r) buildInputs;
      dontConfigure = true;
      dontFixup = true;
      dontInstall = true;
      buildPhase = ''
        runHook preBuild
        mkdir -p $TMPDIR/cache tools/nix
        cp ${pinLines} tools/nix/sources.txt
        cp -RP ${cache}/. $TMPDIR/cache/
        export NONOS_SRC_CACHE=$TMPDIR/cache
        export NONOS_ZIG=${zig}/zig NONOS_CMAKE=${pins.cmake}/bin/cmake
        ${r.env + lib.concatMapStrings (g: "\nexport NONOS_SRC_${g}=${gitTree g}") r.git}
        export ZIG_GLOBAL_CACHE_DIR=$TMPDIR/zig-global ZIG_LOCAL_CACHE_DIR=$TMPDIR/zig-local
        # The script sets CC and the rest for the cross compile as plain shell
        # variables, and builds a host Python and a host ncurses with the
        # compiler the environment names. The build environment exports CC,
        # which would turn those assignments into exports and hand zig to the
        # host builds, so the host toolchain is named only by PATH, as it is on
        # a workstation.
        unset CC CXX AR RANLIB LD NM STRIP OBJCOPY OBJDUMP SIZE STRINGS
        mkdir -p $out
        sh tools/nonos-linux-userland-build ${name} $out
        # The log names this build's paths; it is not part of the artifact.
        rm -f $out/${name}.build.log
        runHook postBuild
      '';
    };

  programs = lib.mapAttrs (n: _: program n) recipes;

  # The Linux personality's built-in BusyBox, from source: the pinned release
  # tarball, the committed config, static against musl, by
  # tools/nonos-busybox-build with the pinned zig as its compiler. zig carries
  # the Linux UAPI headers its musl target needs, so no host headers are read.
  # An aarch64 Linux builder strips BusyBox with an x86_64 strip: its own
  # cannot read the x86_64 ELF it has just linked. x86_64 Linux and macOS
  # build as they always have, so nothing changes there.
  nativeX86 = !(pkgs.stdenv.hostPlatform.isLinux && pkgs.stdenv.hostPlatform.isAarch64);
  x86Binutils = pkgs.pkgsCross.gnu64.buildPackages.binutils-unwrapped;

  busybox = pkgs.stdenv.mkDerivation {
    name = "nonos-busybox-1.36.1";
    src = src.busybox;
    nativeBuildInputs = [ pkgs.gnumake pkgs.perl pkgs.which pkgs.bzip2 ];
    tarball = pkgs.fetchurl {
      url = "https://busybox.net/downloads/busybox-1.36.1.tar.bz2";
      sha256 = "b8cc24c9574d809e7279c3be349795c5d5ceb6fdf19ca709f80cde50e47de314";
    };
    dontConfigure = true;
    dontFixup = true;
    buildPhase = ''
      export HOME=$TMPDIR ZIG_GLOBAL_CACHE_DIR=$TMPDIR/zig ZIG_LOCAL_CACHE_DIR=$TMPDIR/zig
      # BusyBox's link line asks GNU ld for common-symbol warnings and section
      # sorting and a link map, which zig's lld does not take; none changes code.
      cat > $TMPDIR/musl-cc <<EOF
      #!/bin/sh
      for a in "\$@"; do
        shift
        case "\$a" in
          -Wl,--warn-common|-Wl,--sort-common|-Wl,--sort-section,alignment|-Wl,--verbose|-Wl,-Map,*) ;;
          *) set -- "\$@" "\$a" ;;
        esac
      done
      exec ${zig}/zig cc -target x86_64-linux-musl "\$@"
      EOF
      chmod +x $TMPDIR/musl-cc${lib.optionalString (!nativeX86) ''

        mkdir -p $TMPDIR/x86bin
        ln -s ${x86Binutils}/bin/x86_64-unknown-linux-gnu-strip $TMPDIR/x86bin/strip
        export PATH=$TMPDIR/x86bin:$PATH''}
      mkdir -p $TMPDIR/kheaders $out
      # One strip on every host: llvm-strip from the pinned LLVM reads and
      # writes x86_64 ELF the same way on Linux and macOS (see the script).
      STRIP=${pins.llvm.llvm}/bin/llvm-strip \
      CC=$TMPDIR/musl-cc KHEADERS=$TMPDIR/kheaders BUSYBOX_TARBALL=$tarball \
        sh tools/nonos-busybox-build $out/busybox
    '';
    installPhase = "true";
  };

  # The other names the programs answer to, in the table the Linux
  # personality follows; the store carries it at /linux/etc/nonos-links
  # (userland/linux_userland/Userland.mk).
  links = pkgs.writeTextDir "nonos-links" (builtins.readFile ../../userland/linux_userland/nonos-links);
in
{
  inherit zig llama sources programs busybox;
  all = pkgs.symlinkJoin {
    name = "nonos-linux-userland";
    paths = builtins.attrValues programs ++ [ links ];
  };
}
