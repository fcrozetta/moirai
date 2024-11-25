# Set minimum CMake version
cmake_minimum_required(VERSION 3.15)

# Project name and C++ standard
project(moirai LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include directories
include_directories(include)
include_directories(${CMAKE_SOURCE_DIR}/moirai/.venv/lib/python3.13/site-packages/pybind11/include)

# Find Python
find_package(Python3 REQUIRED COMPONENTS Development Interpreter)
include_directories(${Python3_INCLUDE_DIRS})
link_directories(${Python3_LIBRARY_DIRS})

# Set the output directory for binaries and libraries
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/dist)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/dist)

# Define source files
file(GLOB_RECURSE SRCS src/*.cpp)

# Separate main.cpp from other sources
list(FILTER SRCS EXCLUDE REGEX "src/main.cpp")

# Create library
add_library(moirai_lib ${SRCS})

# Create executable for debug and development
add_executable(moirai src/main.cpp ${SRCS})

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

# Set policy and find pybind11
cmake_policy(SET CMP0148 NEW)
set(PYBIND11_FINDPYTHON ON)
find_package(pybind11 REQUIRED)

# Build the Python module
if(CMAKE_BUILD_TYPE STREQUAL "Release")
    add_library(moirai_module MODULE src/bindings.cpp)
    set_target_properties(moirai_module PROPERTIES PREFIX "" SUFFIX ".so")
    target_link_libraries(moirai_module PRIVATE pybind11::module moirai_lib ${Python3_LIBRARIES})
    target_include_directories(moirai_module PRIVATE ${CMAKE_SOURCE_DIR}/src ${CMAKE_SOURCE_DIR}/moirai/.venv/lib/python3.13/site-packages/pybind11/include ${Python3_INCLUDE_DIRS})
    add_custom_command(TARGET moirai_module POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:moirai_module> ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
        COMMAND ${Python3_EXECUTABLE} -m pybind11_stubgen moirai_module --output-dir ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/stubs
        COMMAND ${CMAKE_COMMAND} -E copy_directory ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/stubs ${CMAKE_SOURCE_DIR}/moirai/moirai/stubs)
endif()

# Link Python library
target_link_libraries(moirai_lib PRIVATE Python3::Python)
target_link_libraries(moirai PRIVATE Python3::Python)
