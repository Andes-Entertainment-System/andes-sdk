target_compile_definitions(executable.bin PUBLIC __ANDES_WASM__)
target_compile_definitions(executable.bin PUBLIC _GNU_SOURCE)
set (CMAKE_CXX_FLAGS "-Og")
set (CMAKE_EXE_LINKER_FLAGS
  "-Wl,--max-memory=7340032 -z stack-size=524288   \
   -Wl,--export=I_setup                \
   -Wl,--export=I_process                \
   -Wl,--allow-undefined   \
   -fno-exceptions"
)