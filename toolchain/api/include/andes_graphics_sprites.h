#pragma once

#include <andes_res_types.h>
#include <stdint.h>
#include <stdlib.h>

struct Sprite {
  struct {
    uint32_t width;
    uint32_t height;
  } I_source;
  void* I_slotPtr;
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
};

/**
 * Defines a sprite to be used with SPR functions. Struct members prefixed with 'I_' are internals and should not be
 * tinkered with.
 */
typedef struct Sprite Sprite;

void SPR_loadSpriteSet(SpriteSetResource* res);
void SPR_unloadSpriteSet(SpriteSetResource* res);
extern void SPR_addSprite(Sprite* sprite);
extern void SPR_sortSprite(Sprite* sprite);
extern void SPR_removeSprite(Sprite* sprite);
extern void SPR_removeAllSprites();

extern void I_SPR_setSpriteDataPtr(Sprite* sprite, void* data);
void SPR_updateSpriteFrame(Sprite* sprite);
