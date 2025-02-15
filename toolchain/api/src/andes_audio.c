#include <andes_audio.h>

void SFX_playMusic(struct MusicResource* res) { I_SFX_playMusicFromDisk(res->address, res->size); }

void SFX_playSound(uint8_t channel, struct SoundResource* res) {
  I_SFX_playSoundFromBuffer(channel, res->data, res->dataSize);
}

void SFX_loadSound(struct SoundResource* res) {
  if (res->data != NULL) return;

  res->data = malloc(res->size);
  STO_copyDiskToPtr(res->data, res->address, res->size);
}

void SFX_unloadSound(struct SoundResource* res) {
  if (res->data == NULL) return;

  free(res->data);
  res->data = NULL;
}