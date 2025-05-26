#include <andes_graphics_sprites.h>
#include <andes_storage.h>

void SPR_loadSpriteSet(struct SpriteSetResource* res) {
  if (res->data != NULL) return;

  res->data = malloc(res->size);
  STO_copyDiskToPtr(res->data, res->address, res->size);
}

void SPR_unloadSpriteSet(struct SpriteSetResource* res) {
  if (res->data == NULL) return;

  free(res->data);
  res->data = NULL;
}

void SPR_updateSpriteFrame(struct Sprite* sprite) {
  if (sprite->set == NULL) return;

  const struct SpriteSetFrame* frame = &sprite->set->frames[sprite->frame];
  I_SPR_setSpriteDataPtr(sprite, sprite->set->data + frame->offset);
  sprite->I_source.width = frame->width;
  sprite->I_source.height = frame->height;
}