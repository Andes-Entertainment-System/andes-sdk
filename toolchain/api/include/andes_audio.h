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
 * @brief Set the music channel's volume (0 to 1).
 */
extern void SFX_setMusicVolume(float volume);

/**
 * @brief Set the music channel's panning (-1 to 1, 0 being middle)
 */
extern void SFX_setMusicPanning(float panning);

/**
 * @brief Get the music track's current playback position in samples.
 */
extern uint64_t SFX_getMusicPosition();

/**
 * @brief Returns `true` if music is currently playing.
 */
extern bool SFX_musicPlaying();

/**
 * @brief Set the start position in samples for the looping part of the track that's currently playing. Setting
 * loopStart to -1 or any other negative number will disable looping. Be aware that loopStart is also set automatically
 * by SFX_playMusic, using the loop start specified for the track in the resource config file.
 */
extern void SFX_setMusicLoopStart(int64_t loopStart);

void SFX_loadSound(AudioResource* res);
void SFX_unloadSound(AudioResource* res);

/**
 * @brief Set the specified SFX channel's volume (0 to 1).
 */
extern void SFX_setChannelVolume(uint8_t channel, float volume);

/**
 * @brief Set the specified SFX channel's panning (-1 to 1, 0 being middle)
 */
extern void SFX_setChannelPanning(uint8_t channel, float panning);

/**
 * @brief Enable or disable queuing for the specified SFX channel.
 */
extern void SFX_setChannelQueueEnabled(uint8_t channel, bool enable);

/**
 * @brief Returns `true` if sound is currently playing at the given SFX channel.
 */
extern bool SFX_soundPlaying(uint8_t channel);

/**
 * @brief Play a sound on the specified SFX channel. If -1 (or any other out-of-range number) is given as the channel
 *        ID, the sound will play on the channel with the smallest ID that isn't currently playing a sound already
 *        (or 0 if all channels are playing sounds).
 */
void SFX_playSound(int8_t channel, AudioResource* res);

/**
 * @brief Queue a sound on the specified SFX channel, to be played after the current sound. Only one buffer can be
 *        queued at a time, so if this function is called more than once before the current sound finishes playing,
 *        the queue is overwritten.
 */
void SFX_queueSound(int8_t channel, AudioResource* res);

/**
 * @brief Play a sound from a buffer `src` with a size of `n` bytes containing raw audio data, on the specified SFX
 *        channel. If -1 (or any other out-of-range number) is given as the channel ID, the sound will play on the
 *        channel with the smallest ID that isn't currently playing a sound already (or 0 if all channels are playing
 *        sounds).
 */
extern void SFX_playSoundFromBuffer(int8_t channel, void* src, uint32_t n);

/**
 * @brief Queue a sound from a buffer `src` with a size of `n` bytes containing raw audio data, on the specified SFX
 *        channel, to be played after the current sound. Only one buffer can be queued at a time, so if this function
 *        is called more than once before the current sound finishes playing, the queue is overwritten.
 */
extern void SFX_queueSoundFromBuffer(int8_t channel, void* src, uint32_t n);

/**
 * @brief Returns `true` if there is a sound queued at the given SFX channel.
 */
extern bool SFX_soundQueued(uint8_t channel);

/**
 * @brief Play Opus audio data located at `srcAddr` in the app file, with a size of `n` bytes.
 */
extern void I_SFX_playMusicFromDisk(uint32_t srcAddr, uint32_t n);

/**
 * @brief Load and decode Opus audio data located at `srcAddr` in the app file, with a size of `n` bytes. The size of
 *        the resulting buffer containing the raw audio data is stored on `outSize`.
 */
extern void* I_SFX_loadSoundFromDisk(uint32_t srcAddr, uint32_t n, uint32_t* outSize);