ifeq ($(project),)
PROJECT_NAME                            := $(notdir $(PWD))
else
PROJECT_NAME                            := $(project)
endif
export PROJECT_NAME

##	:
## REF∶
## https∶//doc.rust-lang.org/cargo/reference/profiles.html#custom-profiles

ifneq ($(quiet),)
QUIET=-q
else
QUIET=
endif
export QUIET

ifeq ($(profile),)
PROFILE=dev
CARGO_PROFILE_RELEASE_DEBUG=0
else
# PROFILE=release-with-debug
PROFILE=$(profile)
CARGO_PROFILE_RELEASE_DEBUG=1
endif
export PROFILE
export CARGO_PROFILE_RELEASE_DEBUG

OS                                      :=$(shell uname -s)
export OS
OS_VERSION                              :=$(shell uname -r)
export OS_VERSION
ARCH                                    :=$(shell uname -m)
export ARCH
ifeq ($(ARCH),x86_64)
TRIPLET                                 :=x86_64-linux-gnu
export TRIPLET
endif
ifeq ($(ARCH),arm64)
TRIPLET                                 :=aarch64-linux-gnu
export TRIPLET
endif
ifeq ($(ARCH),arm64)
TRIPLET                                 :=aarch64-linux-gnu
export TRIPLET
endif

ifeq ($(reuse),true)
REUSE                                   :=-r
else
REUSE                                   :=
endif
export REUSE
ifeq ($(bind),true)
BIND                                   :=-b
else
BIND                                   :=
endif
export BIND

ifeq ($(token),)
GITHUB_TOKEN                            :=$(shell cat ~/GITHUB_TOKEN.txt || echo "0")
else
GITHUB_TOKEN                            :=$(shell echo $(token))
endif
export GITHUB_TOKEN

export $(cat ~/GITHUB_TOKEN) && make act

PYTHON                                  := $(shell which python)
export PYTHON
PYTHON2                                 := $(shell which python2)
export PYTHON2
PYTHON3                                 := $(shell which python3)
export PYTHON3

PIP                                     := $(shell which pip)
export PIP
PIP2                                    := $(shell which pip2)
export PIP2
PIP3                                    := $(shell which pip3)
export PIP3

PYTHON_VENV                             := $(shell python -c "import sys; sys.stdout.write('1') if hasattr(sys, 'base_prefix') else sys.stdout.write('0')")
PYTHON3_VENV                            := $(shell python3 -c "import sys; sys.stdout.write('1') if hasattr(sys, 'real_prefix') else sys.stdout.write('0')")

python_version_full := $(wordlist 2,4,$(subst ., ,$(shell python3 --version 2>&1)))
python_version_major := $(word 1,${python_version_full})
python_version_minor := $(word 2,${python_version_full})
python_version_patch := $(word 3,${python_version_full})

my_cmd.python.3 := $(PYTHON3) some_script.py3
my_cmd := ${my_cmd.python.${python_version_major}}

PYTHON_VERSION                         := ${python_version_major}.${python_version_minor}.${python_version_patch}
PYTHON_VERSION_MAJOR                   := ${python_version_major}
PYTHON_VERSION_MINOR                   := ${python_version_minor}

export python_version_major
export python_version_minor
export python_version_patch
export PYTHON_VERSION

-:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?##/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
help:## 	help
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'
rustup-install:## 	rustup-install
	$(shell echo which rustup) || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --no-modify-path --default-toolchain nightly --profile default
	$(shell echo which rustup) && rustup default nightly

cargo-b:## 	cargo-b
	@type -P rustc || $(MAKE) rustup-install
	cargo b
cargo-b-release:## 	cargo-b-release
	@type -P rustc || $(MAKE) rustup-install
	cargo build --release
cargo-c:## 	cargo-c
	@type -P rustc || $(MAKE) rustup-install
	cargo c
install:cargo-install## 	install
cargo-i:## 	cargo-i
	@type -P rustc || $(MAKE) rustup-install
	cargo install --path .

.PHONY:BQS97
BQS97:## 	BQS97
	pdftohtml -c BQS97/byzquorums-distcomputing1998.pdf BQS97/byzquorums-distcomputing1998.html

-include Makefile
-include cargo.mk
-include act.mk
