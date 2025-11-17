start-services:
	$(eval STARTING_BLOCK := $(shell cast block-number -r https://ethereum-rpc.publicnode.com))
	@echo "Using $(STARTING_BLOCK) as start block (Ethereum Mainnet)"
	NEXT_BLOCK=$(STARTING_BLOCK) docker compose up -d

.PHONY: start-services

stop-services:
	docker compose down -v
.PHONY: stop-services
