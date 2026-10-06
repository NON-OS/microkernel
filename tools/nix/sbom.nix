# The software bill of materials: every input the build reads, by version and
# hash, in CycloneDX 1.5. It is computed from the pins themselves (the lock
# files, sources.txt, git-sources.json and flake.lock), so it lists what the
# build is held to rather than what one run happened to fetch, and it is the
# same file on every machine.
{ pkgs, lib, pins, userland, lockFiles, self }:
let
  packagesOf = f: builtins.filter (p: p ? source) ((builtins.fromTOML (builtins.readFile f)).package or [ ]);
  all = lib.concatMap packagesOf lockFiles;
  crates = builtins.attrValues (builtins.listToAttrs (map (p: lib.nameValuePair "${p.name}@${p.version}@${p.source}" p) all));

  crate =
    p:
    let
      git = builtins.match "git\\+([^?#]+).*#(.*)" p.source;
    in
    {
      type = "library";
      name = p.name;
      version = p.version;
      purl = "pkg:cargo/${p.name}@${p.version}";
    }
    // (if git == null then {
      hashes = [ { alg = "SHA-256"; content = p.checksum; } ];
    } else {
      externalReferences = [ { type = "vcs"; url = "${builtins.head git}#${builtins.elemAt git 1}"; } ];
    });

  source = n: s: {
    type = if lib.hasPrefix "zig-" n || lib.hasPrefix "cmake-" n then "application" else "library";
    name = n;
    externalReferences = [ { type = "distribution"; url = s.url; } ];
  } // lib.optionalAttrs (builtins.match "[0-9a-f]{64}" s.hash != null) {
    hashes = [ { alg = "SHA-256"; content = s.hash; } ];
  };

  lock = lib.importJSON (self + "/flake.lock");
  flakeInput = n: v: {
    type = "framework";
    name = n;
    version = v.locked.rev;
    externalReferences = [ { type = "vcs"; url = "https://github.com/${v.locked.owner}/${v.locked.repo}"; } ];
    properties = [ { name = "nix:narHash"; value = v.locked.narHash; } ];
  };

  toolchain = [
    { type = "application"; name = "rust"; version = pins.rust.version; properties = [ { name = "pin"; value = "rust-toolchain.toml"; } ]; }
    { type = "application"; name = "cmake"; version = pins.cmake.version; }
    { type = "application"; name = "clang"; version = pins.llvm.clang-unwrapped.version; }
    { type = "application"; name = "python"; version = pins.python.version; }
  ];

  bom = {
    bomFormat = "CycloneDX";
    specVersion = "1.5";
    version = 1;
    metadata = {
      component = { type = "operating-system"; name = "nonos"; version = lib.removeSuffix "\n" (builtins.readFile (self + "/VERSION")); };
      properties = [ { name = "nonos:inputs"; value = "Cargo.lock files, tools/nix/sources.txt, tools/nix/git-sources.json, flake.lock"; } ];
    };
    components = map crate crates
      ++ lib.mapAttrsToList source userland.sources
      ++ lib.mapAttrsToList flakeInput (lib.filterAttrs (n: v: v ? locked) lock.nodes)
      ++ toolchain;
  };
in
pkgs.writeText "nonos.cdx.json" (builtins.toJSON bom)
