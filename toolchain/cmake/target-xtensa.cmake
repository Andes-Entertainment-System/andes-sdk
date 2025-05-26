# include($ENV{IDF_PATH}/tools/cmake/idf.cmake)

add_compile_definitions("_GNU_SOURCE")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -specs=sim.elf.specs -O2 -fno-jump-tables")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -specs=sim.elf.specs -fno-jump-tables")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-r -nostartfiles -nodefaultlibs -nostdlib -g")
# target_link_libraries(app.bin idf::newlib)