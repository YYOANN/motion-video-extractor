

TARGET=target/debug/motion-photo-extractor

build:
	cargo build

build-release:
	cargo build --release

watch:
	@#find src/ -type f  | entr -c cargo build
	find src/ -type f  | entr -c cargo check

run-watch:
	@echo $(TARGET) \
		| entr -c bash -c '$(TARGET)'
