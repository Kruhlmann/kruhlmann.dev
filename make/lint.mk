ESLINT ?= ./node_modules/.bin/eslint
SVELTE_CHECK ?= ./node_modules/.bin/svelte-check
PRETTIER ?= ./node_modules/.bin/prettier

lint: node_modules ## Lints the source using prettier, eslint and svelte-check.
	@$(PRETTIER) --check $(SRC_DIR) $(TEST_DIR)
	@$(ESLINT) -c .eslintrc.js $(SRC_DIR) $(TEST_DIR) --no-error-on-unmatched-pattern
	@$(SVELTE_CHECK)

fix: node_modules ## Fixes source code problems with prettier, eslint and pre-commit.
	@$(PRETTIER) --write $(SRC_DIR) $(TEST_DIR)
	@$(ESLINT) -c .eslintrc.js $(SRC_DIR) $(TEST_DIR) --no-error-on-unmatched-pattern --fix || true
