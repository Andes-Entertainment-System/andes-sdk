#pragma once
#include <andes_res_types.h>
#include <stdint.h>
#include <stdlib.h>

struct Sprite {
  struct {
    uint32_t dataPtr;
    uint32_t width;
    uint32_t height;
  } I_source;
  struct SpriteSetResource* set;
  uint32_t frame;
  struct {
    int32_t x;
    int32_t y;
  } position;
  struct {
    bool visible;
    bool priority;
    bool hFlip;
    bool vFlip;
  } flags;
  int32_t zIndex;
  uint32_t I_prev;
  uint32_t I_next;
};

void SPR_loadSpriteSet(struct SpriteSetResource* res);
void SPR_unloadSpriteSet(struct SpriteSetResource* res);
extern void SPR_addSprite(struct Sprite* sprite);
extern void SPR_sortSprite(struct Sprite* sprite);
extern void SPR_removeSprite(struct Sprite* sprite);
extern void SPR_removeAllSprites();

extern void I_SPR_setSpriteDataPtr(struct Sprite* sprite, void* data);
void SPR_updateSpriteFrame(struct Sprite* sprite);
