DOCKER  := docker run --rm --user "$(id -u)":"$(id -g)" --env USER=$$USER --env CARGO_HOME=./ -v "$$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.25.0

build:
	$(DOCKER) cargo build

release:
	$(DOCKER) cargo build --release

run:
	$(DOCKER) cargo run ./test_elf/rv32ui-p-add 80000000

clean:
	$(DOCKER) cargo clean
