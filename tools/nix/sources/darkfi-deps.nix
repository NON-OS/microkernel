# The static x86_64 musl tools and libraries DarkFi's build links, from the
# flake's own pinned nixpkgs. darkfi-build.sh reads the file this writes.
#   nix build --no-link --print-out-paths -f tools/nix/sources/darkfi-deps.nix
let
  lock = (builtins.fromJSON (builtins.readFile ../../../flake.lock)).nodes.nixpkgs.locked;
  # By its narHash, so a store that holds it already fetches nothing.
  nixpkgs = builtins.fetchTarball {
    name = "source";
    url = "https://github.com/${lock.owner}/${lock.repo}/archive/${lock.rev}.tar.gz";
    sha256 = lock.narHash;
  };
  pkgs = import nixpkgs { system = "x86_64-linux"; };
  s = pkgs.pkgsStatic;
  # cpal, under drk's rodio, links libasound. nixpkgs' static alsa-lib pulls
  # its config packages, which the cache lacks; the library alone is enough.
  alsa = s.stdenv.mkDerivation {
    pname = "alsa-lib-static-min";
    inherit (pkgs.alsa-lib) version src;
    configureFlags = [ "--enable-static" "--disable-shared" "--disable-python" "--disable-ucm" "--disable-topology" "--without-debug" ];
    enableParallelBuilding = true;
  };
in
pkgs.writeText "darkfi-musl-env" ''
  cc=${s.stdenv.cc}
  ssl=${s.openssl.out}
  ssldev=${s.openssl.dev}
  alsa=${alsa}
  wabt=${pkgs.wabt}
''
