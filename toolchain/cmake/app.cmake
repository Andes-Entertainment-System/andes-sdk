set(CMAKE_EXPORT_COMPILE_COMMANDS True)

if (NOT DEFINED PROJECT_SOURCES)
  message (FATAL_ERROR "PROJECT_SOURCES is not defined! Project source files must be specified in the \
PROJECT_SOURCES variable.")
endif ()

file (GLOB API_SOURCES ${CMAKE_CURRENT_LIST_DIR}/../api/src/*)
include_directories (${CMAKE_CURRENT_LIST_DIR}/../api/include)

include_directories (resources)

add_executable (executable.bin ${API_SOURCES} ${PROJECT_SOURCES} resources/andes_resources.c)

if (${TARGET} STREQUAL "Wasm")
  include(${CMAKE_CURRENT_LIST_DIR}/target-wasm.cmake)
elseif (${TARGET} STREQUAL "Xtensa")
  include(${CMAKE_CURRENT_LIST_DIR}/target-xtensa.cmake)
endif ()