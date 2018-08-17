ASSIGNMENT_NAME := assignment3
BASE_URL := https://web.stanford.edu/class/cs140e
SUBMISSION_SITE := $(BASE_URL)/assignments/submission/
SUBMIT_TAR := $(ASSIGNMENT_NAME).tar.gz

CS140E_REL_ROOT := ..
REPO_NAMES := 0-blinky 1-shell 2-fs 3-spawn os
QUESTIONS_DIRS := $(shell find . -type d -name "questions")

.PHONY: all check submission clean

all:
	@echo "usage: make [target]"
	@echo
	@echo "available targets:"
	@echo "check          ensure every question is answered"
	@echo "submission     create submission tarball"
	@echo "clean          clean products from all targets"

check:
	@okay=true; \
	for qdir in $(QUESTIONS_DIRS); do \
	    for file in "$${qdir}/"*; do \
		    if ! [ -s "$${file}" ]; then \
			  okay=false; \
			  echo "Question file '$${file}' is empty."; \
		    fi \
	    done \
	done; \
	if ! $$okay; then \
		echo "Questions remain unanswered. Aborting."; \
		exit 1; \
	else \
		echo "All questions appear to be answered."; \
	fi

.FORCE:
$(SUBMIT_TAR): .FORCE
	@rm -f $@
	@cwd="$${PWD}"; \
	for repo in $(REPO_NAMES); do \
	    repo_path="$${cwd}/$(CS140E_REL_ROOT)/$${repo}"; \
	    cd "$${repo_path}"; \
	    if ! [ -z "$$(git status --porcelain)" ]; then \
		    echo "There are uncommited changes in $${repo}! Aborting."; \
			rm -f $@; \
		    exit 1; \
	    else \
			git_files=$$(git ls-files) ; \
			cd "$${repo_path}/.." ; \
			for file in $$git_files; do \
				tar -rf "$${cwd}/$@" "$${repo}/$${file}"; \
			done \
	    fi \
	done
	@gzip -f $@
	@mv $@.gz $@

submission: $(SUBMIT_TAR)
	@echo "Your submission file "$^" was successfully created."
	@echo "Submit it at $(SUBMISSION_SITE)"

clean:
	rm -f $(SUBMIT_TAR)
