rm -rf build
mkdir -p build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
if [ $? -ne 0 ]; then
    echo "CMake failed"
    exit 1
fi
make
if [ $? -ne 0 ]; then
    echo "Make failed"
    exit 1
fi

# Step 2: Generate the .pyi files
cd ..
# source moirai/.venv/bin/activate  # Activate the virtual environment
cd build  # Navigate to the build directory
# pybind11-stubgen moirai_module  # Ensure the module name is correct
# if [ $? -ne 0 ]; then
#     echo "pybind11-stubgen failed"
#     exit 1
# fi



# Step 3: Navigate to the directory containing pyproject.toml and build the project using Hatch
cd ../moirai
hatch build