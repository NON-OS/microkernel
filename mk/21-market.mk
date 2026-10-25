# The signed marketplace catalogue, built by the host tools in mk/20-build.mk.

# The catalogue the market capsule embeds. Signed and verified here when the
# operator seed is present; empty otherwise, which the capsule reads as no
# baseline. The serial is the commit time, so a later build never publishes
# an index older than one already installed.
MARKET_OPERATOR_SEED := .keys/marketplace_operator_ed25519.seed
MARKET_OPERATOR_PUB  := .keys/marketplace_operator_ed25519.pub
MARKET_LINUX_LIST    := userland/capsule_market/linux-packages.txt
# Programs the image ships, the Qwen tiers among them, listed by the BLAKE3
# of the program already in the store; one not built is left out.
MARKET_GUEST_LIST    := userland/capsule_market/linux-guests.json
# Curated Alpine apps, each listed with the box it runs in, authenticated as
# the device authenticates them (docs/userland/marketplace-catalogue.md).
MARKET_CATALOGUE     := userland/capsule_market/linux-catalogue.json
MARKET_CATALOGUE_LOCK := userland/capsule_market/linux-catalogue.lock.json
MARKET_INDEX_BIN     := $(TARGET_DIR)/market/index.bin
# The index pins the Qwen runner by its BLAKE3, so the runner is built first.
$(MARKET_INDEX_BIN): $(LINUX_USERLAND_OUT)/qwenchat

$(MARKET_INDEX_BIN): $(MARKETPLACE_INDEX_TOOL) $(MARKET_OPERATOR_PUB) $(MARKET_LINUX_LIST) \
		$(MARKET_GUEST_LIST) $(MARKET_CATALOGUE) $(MARKET_CATALOGUE_LOCK) tools/nonos-market-index tools/nonos-market-catalogue \
		$(wildcard tools/nonos_market_catalogue/*.py) $(wildcard tools/nonos_linux_pack/*.py) \
		$(wildcard $(MARKET_OPERATOR_SEED))
	@$(NONOS_PYTHON) tools/nonos-market-index --out $@ --cli $(MARKETPLACE_INDEX_TOOL) \
		--seed $(MARKET_OPERATOR_SEED) --pubkey $(MARKET_OPERATOR_PUB) \
		--linux-list $(MARKET_LINUX_LIST) --guest-list $(MARKET_GUEST_LIST) \
		--catalogue $(MARKET_CATALOGUE) \
		--serial $$(git log -1 --format=%ct)
