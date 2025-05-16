if (NOT DEFINED WASI_SDK_DIR)
    set (WASI_SDK_DIR               "/opt/wasi-sdk")
endif ()

set (CMAKE_TOOLCHAIN_FILE $ENV{WASI_SDK_DIR}/share/cmake/wasi-sdk.cmake)
set (CMAKE_SYSROOT $ENV{WASI_SDK_DIR}/share/wasi-sysroot)

set (CMAKE_CXX_FLAGS "-O2")

set (CMAKE_EXE_LINKER_FLAGS
    "-nostdlib -Wl,--max-memory=3145728 -z stack-size=524288   \
     -Wl,--no-entry -Wl,--strip-all \
     -Wl,--export=I_setup                \
     -Wl,--export=I_process                \
     -Wl,--export=__heap_base -Wl,--export=__data_end -Wl,--export=malloc -Wl,--export=free \
     -Wl,--allow-undefined   \
     -fno-exceptions"
)

file(GLOB API_SOURCES ${CMAKE_CURRENT_LIST_DIR}/../api/src/*)
include_directories(${CMAKE_CURRENT_LIST_DIR}/../api/include)

include_directories(resources)

add_executable(app.bin ${API_SOURCES} resources/andes_resources.c)
