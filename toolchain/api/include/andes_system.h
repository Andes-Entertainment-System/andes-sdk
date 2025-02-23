#pragma once

#include <stdint.h>

/**
 * @brief Get a pseudo-random 32 bit integer between 0 and the largest possible signed 32 bit value.
 */
extern int32_t SYS_getRandom();

/**
 * @brief Get the amount of time elapsed since console startup, in microseconds.
 */
extern uint64_t SYS_getTime();