# The signed catalogue of the NONOS model repository, which the model fetcher
# (userland/capsule_model_fetch) embeds. tools/nonos-qwen-tier.py builds it
# from the Qwen pins in userland/capsule_linux/src/linux/file/models/, checks
# it against them byte for byte, and signs it with the marketplace operator
# key, as mk/21-market.mk signs the market's index; without the seed it is
# empty, which the fetcher reads as nothing to fetch from.
#
# NONOS_MODEL_MIRROR is the base URL of the NONOS model repository, laid out
# as BASE/TIER/FILE (`nonos-qwen-tier.py mirror` writes that tree). Empty, the
# catalogue names only each file's upstream Qwen URL, and the tool says so.
NONOS_MODEL_MIRROR  ?=
MODEL_CATALOGUE_BIN := $(TARGET_DIR)/models/catalogue.bin
MODEL_MIRROR_STAMP  := $(TARGET_DIR)/models/mirror.txt
MODEL_PINS          := $(wildcard userland/capsule_linux/src/linux/file/models/pinned*.rs)

# The mirror is a variable, not a file, so its value is kept in a stamp that
# is rewritten only when it changes, and the catalogue follows the stamp.
$(shell mkdir -p $(dir $(MODEL_MIRROR_STAMP)); \
	[ -f $(MODEL_MIRROR_STAMP) ] && [ "$$(cat $(MODEL_MIRROR_STAMP))" = "$(NONOS_MODEL_MIRROR)" ] \
	|| printf '%s' "$(NONOS_MODEL_MIRROR)" > $(MODEL_MIRROR_STAMP))

$(MODEL_CATALOGUE_BIN): tools/nonos-qwen-tier.py $(wildcard tools/nonos_qwen_tier/*.py) \
		$(MODEL_PINS) $(MODEL_MIRROR_STAMP) $(MARKET_OPERATOR_PUB) \
		$(wildcard $(MARKET_OPERATOR_SEED))
	@$(NONOS_PYTHON) tools/nonos-qwen-tier.py catalogue --out $@ \
		--seed $(MARKET_OPERATOR_SEED) --pubkey $(MARKET_OPERATOR_PUB) \
		--serial $$(git log -1 --format=%ct) \
		$(if $(NONOS_MODEL_MIRROR),--nonos-mirror $(NONOS_MODEL_MIRROR),)

nonos-mk-model-catalogue: $(MODEL_CATALOGUE_BIN)
.PHONY: nonos-mk-model-catalogue
