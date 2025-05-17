if (NOT DEFINED ENV{WASI_SDK_DIR} AND EXISTS /opt/wasi-sdk)
  set (WASI_SDK_DIR /opt/wasi-sdk)
elseif (NOT DEFINED ENV{WASI_SDK_DIR})
  message(FATAL_ERROR "Cannot find wasm-sdk. Make sure the WASM_SDK_DIR environment variable points to its directory.")
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