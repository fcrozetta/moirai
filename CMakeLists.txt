# Set minimum CMake version
cmake_minimum_required(VERSION 3.15)

# Project name and C++ standard
project(moirai LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include directories
include_directories(include)

# Set the output directory for binaries and libraries
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/dist)

# Define source files
file(GLOB_RECURSE SRCS src/*.cpp)

# Create executable
add_executable(moirai ${SRCS})

# Linker flags
if(APPLE)
    # macOS specific
    target_link_directories(moirai PRIVATE /opt/homebrew/lib)
elseif(WIN32)
    # Windows specific
    target_link_directories(moirai PRIVATE "path/to/windows/libs")
elseif(UNIX)
    # Linux specific
    target_link_directories(moirai PRIVATE /usr/local/lib)
endif()

# Compiler options
if(MSVC)
    # MSVC specific
    target_compile_options(moirai PRIVATE /W4)
else()
    # GCC/Clang specific
    target_compile_options(moirai PRIVATE -Wall -Wextra -pedantic -g)
endif()

# Custom clean target (rename to avoid conflict)
if(UNIX OR MINGW)
    add_custom_target(clean_all COMMAND ${CMAKE_COMMAND} -E remove_directory ${CMAKE_BINARY_DIR}/dist)
endif()
