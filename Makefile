BINARY = erica

BIN_DIR = /usr/local/bin

all:
	cargo build --release && mv target/release/$(BINARY) $(BIN_DIR);

uninstall:
	rm $(BIN_DIR)/$(BINARY);
