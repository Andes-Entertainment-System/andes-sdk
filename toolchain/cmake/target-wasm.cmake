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