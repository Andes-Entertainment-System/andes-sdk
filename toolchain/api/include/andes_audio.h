#pragma once

#include <andes_res_types.h>
#include <stddef.h>
#include <stdlib.h>

#define SFX_SAMPLE_RATE 48000

/**
 * @brief Helper macro to turn a duration in seconds into the corresponding amount of samples.
 */
#define SFX_secondsToSamples(seconds) (seconds * SFX_SAMPLE_RATE)

/**
 * @brief Play a music track. Only one music track can be active at any time.
 */
void SFX_playMusic(AudioResource* res);

/**
 * @brief Pause the current music track being played, or resume it if it's been paused before.
 */
extern void SFX_pauseMusic();

/**
 * @brief Seek the current music track to the specified position in samples.
 */
extern void SFX_seekMusic(uint64_t position);

/**
 * TODO NOT IMPLEMENTED
 * @brief Fade out the current music track within `duration` samples, and then pause it.
 */
extern void SFX_fadeOutMusic(uint64_t duration);

void SFX_loadSound(AudioResource* res);
void SFX_unloadSound(AudioResource* res);

/**
 * TODO NOT IMPLEMENTED
 * @brief Set the specified SFX channel's volume (0 to 1).
 */
extern void SFX_setChannelVolume(uint8_t channel, float volume);

/**
 * TODO NOT IMPLEMENTED
 * @brief Set the specified SFX channel's panning (-1 to 1, 0 being middle)
 */
extern void SFX_setChannelPanning(uint8_t channel, float panning);

/**
 * @brief Play a sound on the specified SFX channel. If -1 (or any other out-of-range number) is given as the channel
 *        ID, the sound will play on the channel with the smallest ID that isn't currently playing a sound already
 *        (or 0 if all channels are playing sounds).
 */
void SFX_playSound(int8_t channel, AudioResource* res);

/**
 * @brief Play Opus audio data located at `srcAddr` in the app file, with a size of `n` bytes.
 */
extern void I_SFX_playMusicFromDisk(uint32_t srcAddr, uint32_t n);

/**
 * @brief Play a sound from a buffer `src` with a size of `n` bytes containing raw audio data, on the specified SFX
 *        channel. If -1 (or any other out-of-range number) is given as the channel ID, the sound will play on the
 *        channel with the smallest ID that isn't currently playing a sound already (or 0 if all channels are playing
 *        sounds).
 */
extern void I_SFX_playSoundFromBuffer(int8_t channel, void* src, uint32_t n);

/**
 * @brief Load and decode Opus audio data located at `srcAddr` in the app file, with a size of `n` bytes. The size of
 *        the resulting buffer containing the raw audio data is stored on `outSize`.
 */
extern void* I_SFX_loadSoundFromDisk(uint32_t srcAddr, uint32_t n, uint32_t* outSize);