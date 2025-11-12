#pragma once

#include <stdint.h>

/**
 * @brief Executes when a channel's queue becomes empty.
 * Since this event occurs within the audio processing thread, it is not synchronous to `process()`.
 */
void EVENT_SFX_onQueueEmpty(uint8_t channel);