## Yaab can be built using glibc or musl. Using musl means the binary will be statically
## linked while using glibc it will be dynamically linked. Default is to build using glibc.
##
TARGET ?= glibc

## help               - Show this help.
.PHONY: help
help:
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

## build-glibc        - Build yaab for x86_64 using glibc
.PHONY: build-glibc
build-glibc:
	cargo build

## build-musl         - Build yaab for x86_64 using musl
.PHONY: build-musl
build-musl:
	cargo build --target x86_64-unknown-linux-musl

## build-release      - Build release using glibc or musl
.PHONY: build-release
build-release:
	./scripts/do_build_release.sh $(TARGET)

## format             - Format the code using rustfmt
.PHONY: format
format:
	cargo fmt

## test               - Run all tests using cargo
.PHONY: test
test:
	cargo test

## install            - Install yaab under $HOME/.cargo using cargo
.PHONY: install
install:
	cargo install --path .

## install-deb        - Install latest locally built yaab. Install it under /usr/bin using deb package
.PHONY: install-deb
install-deb:
	sudo dpkg -i artifacts/yaab.deb

## deb-package        - Create a debian package from the latest release build either using glibc or using musl
.PHONY: deb-package
deb-package: build-release
	./scripts/do_deb_package.sh $(TARGET)

## inc-version        - Increment minor version
.PHONY: inc-version
inc-version:
	./scripts/do_inc_version.sh

## setup-rust         - Setup rust on local machine supports debian/ubuntu
.PHONY: setup-rust
setup-rust:
	./scripts/setup-rust.sh

## setup-docker       - Setup docker on local machine supports debian/ubuntu
.PHONY: setup-docker
setup-docker:
	./scripts/setup-docker.sh

## docker-build       - Build a yaab workspace docker image
.PHONY: docker-build
docker-build:
	(./docker/do_docker_build.sh)

## docker-shell       - Open a yaab workspace docker shell
docker-shell:
	(./docker/do_docker_shell.sh)

## release            - Create a release build, tag and push it to github to trigger a release job
.PHONY: release
release: inc-version
	./scripts/do_build_release.sh $(TARGET)
	./scripts/do_deb_package.sh $(TARGET)
	./scripts/do_release.sh
	git push
	git push --tags

## clean              - Clean
.PHONY: clean
clean:
	cargo clean && rm -r artifacts || true
