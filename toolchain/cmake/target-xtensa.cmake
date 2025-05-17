if (NOT DEFINED ENV{IDF_PATH})
  message(FATAL_ERROR "Cannot find ESP-IDF. Make sure you're building under an ESP-IDF environment. If you installed \
it with the Visual Studio Code ESP-IDF extension and you're currently using VSCode to build your project, you can open \
a terminal under the ESP-IDF environment by opening the Command Prompt (F1) and looking for \"Open ESP-IDF Terminal\". \
Otherwise, you may look at the Installation page in the ESP-IDF docs for further instructions.")
endif ()

include($ENV{IDF_PATH}/tools/cmake/toolchain-esp32s3.cmake)

set(CMAKE_CXX_FLAGS ${CMAKE_CXX_FLAGS} -O2)
set(CMAKE_EXE_LINKER_FLAGS "-Wl,-r -nostartfiles -nodefaultlibs -nostdlib -g")