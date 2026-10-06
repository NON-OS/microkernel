# Get an image

Where a NONOS image comes from, and how to build, seal and check one.

## Released images

This repository holds no image, names no download location and publishes no checksum file for images. NONOS 0.9.2 is installed from an image built from this source tree, as below. If someone hands you an image, ask for the `nonos-release.json` the seal wrote beside it, which records the sha256 of every file it sealed, and compare (see [Check the image](#check-the-image)). That shows the copy matches its record, not who made it: the loader's signature checks at boot do that ([Boot chain and signatures](../security/boot-chain-and-signatures.md)).
