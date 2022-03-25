OUTPUT_FOLDER=dist
STATIC_FILES=$(shell find static/ -type f)
PROJECT_HTML_FILES_SOURCE=$(shell find src/* -type f -name '*.html')
PROJECT_HTML_FILES=$(shell find src/projects/* -type d -exec basename {} \; | awk '{ print "$(OUTPUT_FOLDER)/projects/"$$1".html" }')

all: $(PROJECT_HTML_FILES) $(OUTPUT_FOLDER)/index.html $(OUTPUT_FOLDER)/index.css $(OUTPUT_FOLDER)/index.js

$(OUTPUT_FOLDER)/%.css: src/%.scss Makefile $(OUTPUT_FOLDER)
	sass $< >$@

$(OUTPUT_FOLDER)/%.js: src/%.js Makefile $(OUTPUT_FOLDER)
	cat $< >$@

$(OUTPUT_FOLDER)/%.html: src/%.template.html $(PROJECT_HTML_FILES_SOURCE)
	./scripts/build_html_index $< $(OUTPUT_FOLDER)

$(OUTPUT_FOLDER)/projects/%.html: src/projects/% src/projects/%/definition.json src/projects/%/description.md src/projects/%/badges.md Makefile scripts/build_html scripts/lib $(OUTPUT_FOLDER) $(PROJECT_HTML_FILES_SOURCE)
	./scripts/build_html $< $(OUTPUT_FOLDER)/projects/

$(OUTPUT_FOLDER): $(STATIC_FILES)
	rm -rf $(OUTPUT_FOLDER)
	cp -r static $(OUTPUT_FOLDER)
	rm -f $(OUTPUT_FOLDER)/projects/.gitkeep
