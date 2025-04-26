#include <andes_audio.h>
#include <andes_storage.h>

void SFX_playMusic(struct AudioResource* res) { I_SFX_playMusicFromDisk(res->address, res->size); }

void SFX_playSound(int8_t channel, struct AudioResource* res) {
  I_SFX_playSoundFromBuffer(channel, res->data, res->dataSize);
}

void SFX_loadSound(struct AudioResource* res) {
  if (res->data != NULL) return;

  res->data = I_SFX_loadSoundFromDisk(res->address, res->size, &res->dataSize);
}

void SFX_unloadSound(struct AudioResource* res) {
  if (res->data == NULL) return;

  free(res->data);
  res->data = NULL;
}