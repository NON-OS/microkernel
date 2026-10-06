# Cargo dependencies, offline, from the lock files themselves.
#
# The tree has no workspace: every crate keeps its own Cargo.lock. Each lock
# already names every dependency by name, version and sha256, so the lock is
# the pin and nothing else needs one. Every crate becomes its own fixed output
# derivation keyed by that sha256, fetched once and shared by every lock that
# names it. A build gets a vendor directory holding exactly its own lock's
# crates (and, for -Zbuild-std, those of the standard library's lock), so a
# change to one crate's lock rebuilds that crate and nothing else.
#
# Git dependencies are pinned by commit in their lock. STARKs, the one the
# trust path depends on, comes from the flake's `starks` input, and every lock
# must name that input's commit (tools/nonos-starks-sync). Any other git
# dependency is pinned by tree hash in git-sources.json.
{ pkgs, lib, rust, self, starks }:
let
  gitPins = (lib.importJSON ./git-sources.json).sources;

  parseGit =
    src:
    let
      parts = builtins.match "git\\+([^?]+)(\\?(rev|tag|branch)=([^#]*))?#(.*)" src;
    in
    if parts == null then
      null
    else
      {
        url = builtins.head parts;
        type = builtins.elemAt parts 2;
        value = builtins.elemAt parts 3;
        sha = builtins.elemAt parts 4;
      };

  registryCrate =
    p:
    let
      tarball = pkgs.fetchurl {
        name = "crate-${p.name}-${p.version}.tar.gz";
        url = "https://static.crates.io/crates/${p.name}/${p.name}-${p.version}.crate";
        sha256 = p.checksum;
      };
    in
    pkgs.runCommand "crate-${p.name}-${p.version}" { } ''
      mkdir $out
      tar xf ${tarball} -C $out --strip-components=1
      printf '{"files":{},"package":"${p.checksum}"}' > $out/.cargo-checksum.json
    '';

  starksUrl = "https://github.com/NON-OS/STARKs.git";

  gitTree =
    sha:
    let
      pin = gitPins.${sha} or null;
    in
    if sha == starks.rev then
      starks
    else if pin == null || pin.hash == null then
      null
    else
      pkgs.fetchgit {
        inherit (pin) url hash;
        rev = sha;
      };

  replaceWorkspaceValues = pkgs.writers.writePython3 "replace-workspace-values" {
    libraries = with pkgs.python3Packages; [ tomli tomli-w ];
    flakeIgnore = [ "E501" "W503" ];
  } (builtins.readFile (pkgs.path + "/pkgs/build-support/rust/replace-workspace-values.py"));

  # A git dependency, vendored whole. A crate in a git workspace may read its
  # siblings' sources (nox_verify takes stark_proofs' wire format through
  # #[path = "../../stark_proofs/..."]), which builds from a checkout but not
  # from the one directory `cargo vendor` would copy. So the whole tree is
  # kept, every crate in it gets its workspace-inherited fields written in and
  # an empty checksum, as cargo vendor leaves a crate, and `links` names the
  # directory each name-version lives in.
  gitVendor =
    g:
    let
      inherit (g) sha;
      tree = gitTree sha;
      pin = gitPins.${sha} or { url = "an unpinned repository"; };
      stale = g.url == starksUrl;
    in
    if tree == null && stale then
      pkgs.runCommand "stale-starks-${builtins.substring 0 7 sha}" { } ''
        echo "a Cargo.lock names STARKs at ${sha}, and flake.lock pins ${starks.rev}." >&2
        echo "Run tools/nonos-starks-sync in nix develop." >&2
        exit 1
      ''
    else if tree == null then
      pkgs.runCommand "missing-git-${builtins.substring 0 7 sha}" { } ''
        echo "a Cargo.lock names ${pin.url} at ${sha}," >&2
        echo "and tools/nix/git-sources.json has no hash for that commit." >&2
        echo "Run tools/nonos-nix-pin on a machine that can fetch it." >&2
        exit 1
      ''
    else
      pkgs.runCommand "git-vendor-${builtins.substring 0 7 sha}" { nativeBuildInputs = [ rust pkgs.jq ]; } ''
        export CARGO_HOME=$TMPDIR/cargo-home
        cp -rL ${tree} $out
        chmod -R u+w $out
        : > $TMPDIR/links
        for m in $(cd $out && find . -name Cargo.toml -not -path '*/target/*' | LC_ALL=C sort); do
          meta=$(cargo metadata --offline --format-version 1 --no-deps --manifest-path "$out/$m" 2>/dev/null) || continue
          root=$(echo "$meta" | jq -r .workspace_root)
          echo "$meta" | jq -r '.packages[] | "\(.name)-\(.version) \(.manifest_path)"' | while read -r id manifest; do
            [ "$manifest" = "$out/''${m#./}" ] || continue
            dir=$(dirname "$manifest")
            if grep -q workspace "$manifest" && [ "$root/Cargo.toml" != "$manifest" ]; then
              ${replaceWorkspaceValues} "$manifest" "$root/Cargo.toml"
            fi
            printf '{"files":{},"package":null}' > "$dir/.cargo-checksum.json"
            echo "$id ''${dir#$out/}" >> $TMPDIR/links
          done
        done
        sort -u $TMPDIR/links > $out/.nonos-links
      '';

  packagesOf = f: builtins.filter (p: p ? source) ((builtins.fromTOML (builtins.readFile f)).package or [ ]);

  uniqueBy = key: xs: builtins.attrValues (builtins.listToAttrs (map (x: lib.nameValuePair (key x) x) xs));

  # The standard library's own lock, which -Zbuild-std resolves against. A
  # copy is committed so reading it is not import-from-derivation; the
  # rust-src-lock check holds it to the toolchain's.
  rustSrcLock = ./rust-src.Cargo.lock;
in
rec {
  inherit rustSrcLock;

  # The vendor directory for a set of lock files, and the cargo configuration
  # that points cargo at it. Real copies, so every path rustc sees sits under
  # this one store path and one --remap-path-prefix covers it; a git crate is
  # a link into its repository's tree, which is copied here whole.
  vendor =
    { name, lockFiles, buildStd ? false }:
    let
      all = lib.concatMap packagesOf (lockFiles ++ lib.optional buildStd rustSrcLock);
      registry = uniqueBy (p: "${p.name}-${p.version}") (builtins.filter (p: lib.hasPrefix "registry+" p.source) all);
      git = uniqueBy (p: "${p.name}-${p.version}-${(parseGit p.source).sha}") (builtins.filter (p: lib.hasPrefix "git+" p.source) all);
      shas = lib.unique (map (p: (parseGit p.source).sha) git);
      sourceOf = sha: parseGit (lib.findFirst (p: (parseGit p.source).sha == sha) null git).source;
      gitSection =
        sha:
        let
          g = sourceOf sha;
        in
        ''

          [source."nonos-git-${sha}"]
          git = "${g.url}"
          ${lib.optionalString (g.type != null) "${g.type} = \"${g.value}\""}
          replace-with = "nonos-vendor-${sha}"

          [source."nonos-vendor-${sha}"]
          directory = "@out@/git-${sha}"
        '';
      config = ''
        [source.crates-io]
        replace-with = "nonos-vendor"

        [source.nonos-vendor]
        directory = "@out@/registry"
      '' + lib.concatMapStrings gitSection shas;
    in
    pkgs.runCommand "cargo-vendor-${name}" { inherit config; passAsFile = [ "config" ]; } ''
      mkdir -p $out/registry
      ${lib.concatMapStrings (p: ''
        cp -r ${registryCrate p} $out/registry/${p.name}-${p.version}
      '') registry}
      ${lib.concatMapStrings (sha: ''
        cp -r ${gitVendor (sourceOf sha)} $out/git-tree-${sha}
        mkdir -p $out/git-${sha}
      '') shas}
      ${lib.concatMapStrings (p: let g = parseGit p.source; in ''
        dir=$(awk -v id=${p.name}-${p.version} '$1 == id { print $2; exit }' $out/git-tree-${g.sha}/.nonos-links)
        [ -n "$dir" ] || { echo "no crate ${p.name} ${p.version} in ${g.url} at ${g.sha}" >&2; exit 1; }
        ln -s ../git-tree-${g.sha}/$dir $out/git-${g.sha}/${p.name}-${p.version}
      '') git}
      substitute $configPath $out/config.toml --subst-var out
    '';

  # rustc through this wrapper rewrites every build path it would otherwise
  # write into a binary (the vendor directory, the toolchain's sources, the
  # build directory) to a fixed name, so the bytes do not depend on where or on
  # which operating system the build ran.
  rustcWrapper = pkgs.writeShellScript "nonos-rustc" ''
    rustc=$1; shift
    exec "$rustc" "$@" $NONOS_REMAP
  '';

  # Shell lines that point cargo at a vendor directory, offline and frozen.
  setup = vendorDir: ''
    export CARGO_HOME=$TMPDIR/cargo-home
    mkdir -p $CARGO_HOME
    cat ${vendorDir}/config.toml > $CARGO_HOME/config.toml
    cat >> $CARGO_HOME/config.toml <<EOF

    [net]
    offline = true
    EOF
    export CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 CARGO_TERM_COLOR=never
    export RUSTC_WRAPPER=${rustcWrapper}
    sysroot=$(rustc --print sysroot)
    export NONOS_REMAP="--remap-path-prefix=${vendorDir}=/cargo --remap-path-prefix=$sysroot=/rust --remap-path-prefix=$NIX_BUILD_TOP=/nonos"
  '';
}
