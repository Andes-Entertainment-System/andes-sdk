#include <andes_graphics_sprites.h>
#include <andes_storage.h>

struct SerializedSprite internalSprites[128];
bool spritesUpdated[128];

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

void SPR_addSprite(struct Sprite* sprite) {
  for (int i = 0; i < 128; i++) {
    // if internal sprite is not in use
    if ((internalSprites[i].flags & 1) == 0) {
      sprite->_id = i;
      break;
    }
  }

  SPR_updateSprite(sprite);
}

void SPR_updateSprite(struct Sprite* sprite) {
  struct SerializedSprite* internalSprite = &internalSprites[sprite->_id];
  struct SpriteSetFrame* frame = &sprite->set->frames[sprite->frame];

  internalSprite->flags = sprite->flags.hFlip << 3 | sprite->flags.priority << 2 | sprite->flags.visible << 1 | 1;
  internalSprite->data = (uint8_t*)sprite->set->data + frame->offset;
  internalSprite->width = frame->width;
  internalSprite->height = frame->height;
  internalSprite->xPos = sprite->position.x;
  internalSprite->yPos = sprite->position.y;

  spritesUpdated[sprite->_id] = true;
}

void SPR_removeSprite(struct Sprite* sprite) {
  internalSprites[sprite->_id].flags = 0;
  spritesUpdated[sprite->_id] = true;
}

void I_SPR_onRender() { I_SPR_transferSprites(&internalSprites, &spritesUpdated); }