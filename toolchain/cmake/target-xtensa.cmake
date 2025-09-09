# include($ENV{IDF_PATH}/tools/cmake/idf.cmake)

target_compile_definitions(executable.bin PUBLIC __ANDES_XTENSA__)
target_compile_definitions(executable.bin PUBLIC _GNU_SOURCE)
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -specs=sim.elf.specs -O2 -fno-jump-tables -fno-function-cse")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -specs=sim.elf.specs -fno-jump-tables -fno-function-cse")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-r -nostartfiles -nodefaultlibs -nostdlib -g")
# target_link_libraries(executable.bin idf::newlib)