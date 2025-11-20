
start-services:
	$(eval STARTING_BLOCK := $(shell cast block-number -r $${L1_RPC:-https://ethereum-rpc.publicnode.com}))
	@echo "Using $(STARTING_BLOCK) as start block (Ethereum Mainnet)"
	NEXT_BLOCK=$(STARTING_BLOCK) docker compose -f core-docker-compose.yml up -d
	docker compose -f docker-compose-ethproofs.yml up -d
.PHONY: start-services

start-split-work:
	$(eval STARTING_BLOCK := $(shell cast block-number -r $${L1_RPC:-https://ethereum-rpc.publicnode.com}))
	@echo "Using $(STARTING_BLOCK) as start block (Ethereum Mainnet)"
	NEXT_BLOCK=$(STARTING_BLOCK) docker compose -f core-docker-compose.yml up -d
	docker compose -f docker-compose-split-work.yml up -d
.PHONY: start-services

start-mock-proving:
	$(eval STARTING_BLOCK := $(shell cast block-number -r $${L1_RPC:-https://ethereum-rpc.publicnode.com}))
	@echo "Using $(STARTING_BLOCK) as start block (Ethereum Mainnet)"
	NEXT_BLOCK=$(STARTING_BLOCK) docker compose -f core-docker-compose.yml up -d
	docker compose -f docker-compose-mock.yml up -d
.PHONY: start-services

start-split-work-amd64:
	$(eval STARTING_BLOCK := $(shell cast block-number -r $${L1_RPC:-https://ethereum-rpc.publicnode.com}))
	@echo "Using $(STARTING_BLOCK) as start block (Ethereum Mainnet)"
	NEXT_BLOCK=$(STARTING_BLOCK) docker compose -f core-docker-compose.yml up -d
	docker compose -f docker-compose-split-work-amd64.yml up -d
.PHONY: start-services

stop-services:
	docker compose down --remove-orphans -v
.PHONY: stop-services
