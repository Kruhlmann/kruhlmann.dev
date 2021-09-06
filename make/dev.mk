TSC ?= ./node_modules/.bin/tsc
SAPPER ?= ./node_modules/.bin/sapper

dev: node_modules config/version.json ## Runs the software in development mode with hot reload.
	@$(SAPPER) dev --src $(CORE_DIR) --routes $(ROUTES_DIR)

.PHONY: dev
