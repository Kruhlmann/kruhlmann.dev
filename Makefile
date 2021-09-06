SRC_DIR ?= src
TEST_DIR ?= tests
ROUTES_DIR ?= src/routes
CORE_DIR ?= src/core
VERSION ?= $(shell git describe --always --tag)
PKG_MANAGER ?= yarn
PKG_LOCKFILE ?= yarn.lock
TARGET ?= local

TS_SOURCES ?= $(shell find "$(SRC_DIR)" -type f -name '*.ts')
JS_SOURCES ?= $(shell find "$(SRC_DIR)" -type f -name '*.js')
SVELTE_SOURCES ?= $(shell find "$(SRC_DIR)" -type f -name '*.svelte')
SCSS_SOURCES ?= $(shell find "$(SRC_DIR)" -type f -name '*.scss')
SOURCES ?= $(shell find "$(SRC_DIR)" -type f -name '*.ts' -o -name '*.js' -o -name '*.svelte' -o -name '*.scss')
INTEGRATION_TESTS ?= $(shell find 'tests/integration' -type f -name '*.ts' -o -name '*.js' 2>/dev/null)
UNIT_TESTS ?= $(shell find 'tests/components' -type f -name '*.test.js' -o -name '*.test.ts' 2>/dev/null)

CONFIG_DIR ?= config
SCRIPTS_DIR ?= scripts

JS_SVELTE_FILES ?= $(shell grep '<script>' $(SVELTE_SOURCES) | wc -l)
TS_SVELTE_FILES ?= $(shell grep '<script lang="ts">' $(SVELTE_SOURCES) | wc -l)
BAKCEND_LIBRARY ?= $(shell grep -Eo '"polka"|"express"' package.json)
BUNDLER_LIBRARY ?= $(shell grep -Eo '"webpack"|"rollup"' package.json)

include make/notes.mk
include make/test.mk
include make/build.mk
include make/lint.mk
include make/dev.mk

metadata: ## Prints project metadata.
	@printf "\e[33m%s JavaScript based svelte components\e[0m\n" "$(JS_SVELTE_FILES)"
	@printf "\e[36m%s TypeScript based svelte components\e[0m\n" "$(TS_SVELTE_FILES)"
	@printf "Using \e[1;94m%s\e[0m backend\n" "$(BAKCEND_LIBRARY)"
	@printf "Using \e[1;94m%s\e[0m bundler\n" "$(BUNDLER_LIBRARY)"
	@printf "Using \e[1;94m%s\e[0m for package management stored in \e[1;94m%s\e[0m\n" "$(PKG_MANAGER)" "$(PKG_LOCKFILE)"
	@printf "Core files are stored in \e[1;94m%s\e[0m\n" "$(CORE_DIR)"
	@printf "Sapper routes are stored in \e[1;94m%s\e[0m\n" "$(ROUTES_DIR)"

help: metadata $(SCRIPTS_DIR)/make_help_brief ## Print this help message.
	@printf "\n"
	@grep '\w:.* ##' $(MAKEFILE_LIST) | grep -v "@grep '" | $(SCRIPTS_DIR)/make_help_brief
	@printf "\n\e[1mFor more information on make targets run \e[4;94mmake help-verbose\e[0m\n"

help-verbose: metadata $(SCRIPTS_DIR)/make_help ## Prints a description and dependencies for each make target.
	@printf "\n"
	@grep '\w:.* ##' $(MAKEFILE_LIST) | grep -v "@grep '" | $(SCRIPTS_DIR)/make_help

hooks: node_modules .pre-commit-config.yaml ## Setup hooks.
	@which pre-commit 1>/dev/null || pip install pre-commit
	@pre-commit install

run: __sapper__/build node_modules ## Run the program.
	@node __sapper__/build

clean: ## Clean source directory.
	@rm -rf \
		test-report.xml \
		yarn-error.log \
		coverage/ \
		__sapper__ \

loccount: ## Counts the lines of code in the project.
	@find $(SRC_DIR) $(TEST_DIR) -name '*.svelte' -o -name '*.js'  -o -name '*.ts'  -o -name '*.html'  -o -name '*.scss' | xargs wc -l | tail -n 1 | sed -e 's/ total//' -e 's/ //'

install: node_modules ## Install dependencies.

node_modules: package.json
	@$(PKG_MANAGER) install

config/version.json: force ## Create a version.json file with the current git version.
	@printf '{\n    "version": "$(VERSION)"\n}\n' > config/version.json

upgrade: node_modules ## Upgrade one or more packages.
	@$(PKG_MANAGER) upgrade

force:

.PHONY: clean upgrade hooks force help help-verbose metadata
