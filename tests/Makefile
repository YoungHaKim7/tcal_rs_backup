# Makefile for tcal_rs tests
# Simple test runner for batch test files

BATCH_FILES = $(wildcard *.batch)
BINARY = ../target/release/qcal_rs
CARGO = cargo

.PHONY: all test build clean check run-tests $(BATCH_FILES)

all: test

build:
	$(CARGO) build --release

test: build run-tests

run-tests: $(BATCH_FILES)

check:
	$(CARGO) test

clean:
	$(CARGO) clean

# Run each batch file - use shell to force execution
define run_batch_file
@echo "Running tests from $(1)..."
@FAILED=0; \
PASSED=0; \
TOTAL=0; \
SKIPPED=0; \
while IFS= read -r line || [ -n "$$line" ]; do \
	if [ -z "$$line" ]; then \
		continue; \
	fi; \
	if echo "$$line" | grep -qE '^\s'; then \
		EXPECTED="$$(echo "$$line" | sed 's/^\s*//' | sed 's/\s*$$//')"; \
		OUTPUT="$$(echo "$$INPUT" | $(BINARY) 2>/dev/null)"; \
		RESULT="$$(echo "$$OUTPUT" | tail -1 | sed 's/\s*$$//')"; \
		TOTAL=$$((TOTAL + 1)); \
		if [ "$$RESULT" = "$$EXPECTED" ]; then \
			PASSED=$$((PASSED + 1)); \
		else \
			FAILED=$$((FAILED + 1)); \
			echo "  FAILED: '$$INPUT'"; \
			echo "    Expected: $$EXPECTED"; \
			echo "    Got:      $$RESULT"; \
		fi; \
	else \
		INPUT="$$line"; \
	fi; \
done < $(1); \
echo "  Results: $$PASSED/$$TOTAL passed ($$SKIPPED skipped)"; \
if [ $$FAILED -gt 0 ]; then \
	exit 1; \
fi
endef

# Generate targets for each batch file
$(foreach batch,$(BATCH_FILES),$(eval $(batch): build; $$(call run_batch_file,$(batch))))
