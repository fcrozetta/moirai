# Project name and output directory
OUT_DIR = dist
OUT = $(OUT_DIR)/moirai
MACOS_DIR = $(CONTENTS_DIR)/MacOS
RESOURCES_DIR = $(CONTENTS_DIR)/Resources

# Compiler and linker options
CXX = g++
CXXFLAGS = -std=c++20 -Iinclude

# Default build mode (debug or release)
BUILD = debug

# Platform-specific configuration
UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S), Darwin)          # macOS
    CXXFLAGS += -I/opt/homebrew/include
    LDFLAGS = -L/opt/homebrew/lib
endif

# Source and output
# SRC = src/moirai.cpp
SRC = src/main.cpp
# LOGO = logo.png

# Build commands
all: $(OUT_DIR) $(BUILD)

# Create output directory
$(OUT_DIR):
	mkdir -p $(OUT_DIR)

# Create macOS app bundle for release builds
release: $(OUT_DIR)
	# Create the necessary directories
	mkdir -p $(MACOS_DIR)
	mkdir -p $(RESOURCES_DIR)

	# Compile the application
	$(CXX) $(CXXFLAGS) -O3 $(SRC) $(LDFLAGS) -o $(MACOS_DIR)/moirai

# Debug target (standard executable)
debug: $(OUT_DIR)
	$(CXX) $(CXXFLAGS) -g $(SRC) $(LDFLAGS) -o $(OUT)

# Clean command
clean:
	rm -rf $(OUT_DIR)
