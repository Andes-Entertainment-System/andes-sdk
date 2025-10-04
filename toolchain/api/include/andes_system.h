#pragma once

#include <stdint.h>

/**
 * @brief Get the amount of time elapsed since console startup, in microseconds.
 */
extern uint64_t SYS_getTime();

/**
 * @brief (ONLY AVAILABLE TO SYSTEM MENU) Load the specified app.
 */
extern void I_SYS_loadApp(char* path);