RESET=\033[0m
RED=\033[0;31m
GREEN=\033[0;32m
YELLOW=\033[0;33m
BLUE=\033[0;34m

TARGET:=

define build_release
	@echo "$(BLUE)Building target '$(TARGET)'$(RESET)"...
	cross build --release --target=$(TARGET) --bin async_iot_system_state --features=system_state
	@if [ $?==0 ]; then echo "$(GREEN)'$(TARGET)' successfully built.$(RESET)"; else echo "$(RED)'$(TARGET)' build failed!"; fi
endef

release_armv6_linux: TARGET=arm-unknown-linux-gnueabihf
release_armv6_linux:
	$(build_release)

release_armv7_linux: TARGET=armv7-unknown-linux-gnueabihf
release_armv7_linux:
	$(build_release)

release_aarch64_linux: TARGET=aarch64-unknown-linux-gnu
release_aarch64_linux:
	$(build_release)

release_x86_linux: TARGET=x86_64-unknown-linux-gnu
release_x86_linux:
	$(build_release)

release_x86_windows: TARGET=x86_64-pc-windows-msvc
release_x86_windows:
	$(build_release)

release_x86_darwin: TARGET=x86_64-apple-darwin
release_x86_darwin:
	$(build_release)

release: release_x86_darwin release_x86_linux release_aarch64_linux release_armv7_linux release_armv6_linux
