
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
.PHONY: start-split-work

start-mock-proving:
	$(eval STARTING_BLOCK := $(shell cast block-number -r $${L1_RPC:-https://ethereum-rpc.publicnode.com}))
	@echo "Using $(STARTING_BLOCK) as start block (Ethereum Mainnet)"
	NEXT_BLOCK=$(STARTING_BLOCK) docker compose -f core-docker-compose.yml up -d
	docker compose -f docker-compose-mock.yml up -d
.PHONY: start-mock-proving

start-split-work-amd64:
	$(eval STARTING_BLOCK := $(shell cast block-number -r $${L1_RPC:-https://ethereum-rpc.publicnode.com}))
	@echo "Using $(STARTING_BLOCK) as start block (Ethereum Mainnet)"
	NEXT_BLOCK=$(STARTING_BLOCK) docker compose -f core-docker-compose.yml up -d
	docker compose -f docker-compose-split-work-amd64.yml up -d
.PHONY: start-split-work-amd64

start-multistep-mock-proving:
	$(eval STARTING_BLOCK := $(shell cast block-number -r $${L1_RPC:-https://ethereum-rpc.publicnode.com}))
	@echo "Using $(STARTING_BLOCK) as start block (Ethereum Mainnet)"
	NEXT_BLOCK=$(STARTING_BLOCK) docker compose -f core-docker-compose.yml up -d
	docker compose -f docker-compose-multistep-mock.yml up -d
.PHONY: start-mock-proving


stop-services:
	docker compose \
		-f core-docker-compose.yml \
		-f docker-compose-ethproofs.yml \
		-f docker-compose-split-work.yml \
		-f docker-compose-split-work-amd64.yml \
		-f docker-compose-mock.yml \
		-f docker-compose-multistep-mock.yml \
		down --remove-orphans -v
.PHONY: stop-services
