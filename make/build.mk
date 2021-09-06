SAPPER ?= ./node_modules/.bin/sapper

build: __sapper__/build ## Builds a production version of the software.

export: __sapper__/export ## Exports the production version of the software for static viewing.

__sapper__/build: config/version.json node_modules clean
	@$(SAPPER) build --legacy --src $(CORE_DIR) --routes $(ROUTES_DIR)

__sapper__/export: config/version.json node_modules clean
	@$(SAPPER) export --src $(CORE_DIR) --routes $(ROUTES_DIR)

docker: ## Builds the associated docker image for the software.
	@docker-compose build

.PHONY: __sapper__/build docker
