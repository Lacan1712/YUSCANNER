BINARY_NAME=uscanner
BINARY_ALIAS=uscanner
TARGET=target/release/$(BINARY_NAME)
INSTALL_PATH=/usr/local/bin/$(BINARY_ALIAS)

all: build setcap install

build:
	cargo build --release

setcap:
	sudo setcap cap_net_raw,cap_net_admin=eip $(TARGET)

install:
	sudo cp $(TARGET) $(INSTALL_PATH)
	sudo chmod +x $(INSTALL_PATH)
	@echo "✅ Binário instalado como '$(BINARY_ALIAS)' em $(INSTALL_PATH)"
