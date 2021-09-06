CYPRESS ?= ./node_modules/.bin/cypress
JEST ?= ./node_modules/.bin/jest
JEST_FLAGS ?= --passWithNoTests --verbose --ci --reporters=default --reporters=jest-junit

test: test-unit test-integration ## Runs unit and integration tests.

test-integration: integration-report.xml ## Runs integration tests.

test-integration-interactive: $(INTEGRATION_TESTS) clean node_modules __sapper__/build ## Runs cypress in interractive mode.
	$(SCRIPTS_DIR)/integration_test_interractive $(CYPRESS) $(CONFIG_DIR)/cypress.json

test-unit: test-report.xml ## Runs unit tests.

test-report.xml: $(UNIT_TESTS) clean node_modules
	$(JEST) $(JEST_FLAGS) --config $(CONFIG_DIR)/jest.config.js

integration-report.xml: $(INTEGRATION_TESTS) clean node_modules __sapper__/build
	$(SCRIPTS_DIR)/integration_test $(CYPRESS) $(CONFIG_DIR)/cypress.json

.PHONY: test test-integration integration-test-interractive test-unit
