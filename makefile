# Compiler and flags
CXX = g++
CXXFLAGS = -std=c++20 -Iinclude -MMD
LDFLAGS = -L/opt/homebrew/lib

# Directories
SRC_DIR = src
OBJ_DIR = dist/obj
OUT_DIR = dist
OUT = $(OUT_DIR)/moirai

# Find all source files
SRCS = $(wildcard $(SRC_DIR)/*.cpp)
OBJS = $(SRCS:$(SRC_DIR)/%.cpp=$(OBJ_DIR)/%.o)
DEPS = $(OBJS:.o=.d)

# Targets
all: $(OUT)

$(OUT): $(OBJS)
	$(CXX) $(CXXFLAGS) -g $(OBJS) $(LDFLAGS) -o $@

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp
	mkdir -p $(OBJ_DIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

-include $(DEPS)

clean:
	rm -rf $(OUT_DIR)
