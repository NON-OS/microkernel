# The periodic cache nonos.shield proves from: the top of the launch
# circuit's periodic tree, about 8 MB, the same file the phone apps bundle.
#
# Built here as STARKs says to build it: nox_bench from the commit flake.lock
# pins, with the features shield_core links nox_prover with (not_before,
# parallel), proving the pinned transfer-eth vector once with NOX_CACHE
# naming a file that does not exist yet, so the full proof writes the tree.
# The vector's own entropy makes the proof the pinned one, and nothing else
# reaches the file, so two builds give the same bytes.
#
# Then the file is held to what it must be before anything ships it: a second
# nox_bench run proves the same vector from it, which nox_prover refuses
# unless the file's root is PERIODIC_ROOT, and that proof must be the pinned
# proof.json byte for byte.
{ pins, rustBuild, starks }:
rustBuild {
  name = "nonos-periodic-cache";
  src = starks;
  lockFiles = [ (starks + "/Cargo.lock") ];
  script = ''
    cargo build --frozen --release -p nox_prover --bin nox_bench \
      --features nox_prover/not_before,nox_prover/parallel
    bench=$PWD/target/release/nox_bench
    vec=spec/wallet-vectors-not-before/transfer-eth
    python3 -c 'import sys; sys.stdout.buffer.write(bytes.fromhex(open(sys.argv[1]).read().strip().removeprefix("0x")))' \
      $vec/entropy.hex > $TMPDIR/entropy.bin
    mkdir -p $out
    cache=$out/periodic.top
    NOX_ENTROPY=$TMPDIR/entropy.bin NOX_CACHE=$cache \
      $bench $vec/request.json $vec/seed.json $TMPDIR/built.json
    cmp $TMPDIR/built.json $vec/proof.json
    [ -s $cache ]
    NOX_ENTROPY=$TMPDIR/entropy.bin NOX_CACHE=$cache \
      $bench $vec/request.json $vec/seed.json $TMPDIR/cached.json | tee $TMPDIR/cached.log
    grep -q '^cache     periodic tree skipped' $TMPDIR/cached.log
    cmp $TMPDIR/cached.json $vec/proof.json
    sha256sum $cache | cut -d' ' -f1 > $out/periodic.top.sha256
  '';
}
