#pragma once
#include <andes_res_types.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Get the sprite's width according to the frame and spriteset it's currently using.
 */
#define SPR_SPRITE_WIDTH(sprite) (sprite)->set->frames[(sprite)->frame].width
/**
 * Get the sprite's height according to the frame and spriteset it's currently using.
 */
#define SPR_SPRITE_HEIGHT(sprite) (sprite)->set->frames[(sprite)->frame].height

struct Sprite {
  uint8_t _id;
  struct SpriteSetResource* set;
  uint32_t frame;
  struct {
    int16_t x;
    int16_t y;
  } position;
  struct {
    bool visible;
    bool priority;
    bool hFlip;
  } flags;
};

struct SerializedSprite {
  void* data;
  uint32_t width;
  uint32_t height;
  int32_t xPos;
  int32_t yPos;
  uint8_t flags;
};

void SPR_loadSpriteSet(struct SpriteSetResource* res);
void SPR_unloadSpriteSet(struct SpriteSetResource* res);
void SPR_addSprite(struct Sprite* sprite);
void SPR_updateSprite(struct Sprite* sprite);
void SPR_removeSprite(struct Sprite* sprite);
void SPR_updateAllSprites();
void SPR_removeAllSprites();

void I_SPR_onRender();

extern void I_SPR_transferSprites(struct SerializedSprite (*sprites)[128], bool (*updated)[128]);
