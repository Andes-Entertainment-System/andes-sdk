#pragma once

#include "andes_graphics_tiles.h"
#include "andes_res_types.h"

/**
 * Get the horizontal flip flag of a chunk descriptor.
 */
#define CHUNK_HFLIP_FLAG(chunkDescriptor) ((chunkDescriptor) >> 13 & 1)

/**
 * Get the vertical flip flag of a chunk descriptor.
 */
#define CHUNK_VFLIP_FLAG(chunkDescriptor) ((chunkDescriptor) >> 14 & 1)

/**
 * Get the priority flag of a chunk descriptor.
 */
#define CHUNK_PRIORITY_FLAG(chunkDescriptor) ((chunkDescriptor) >> 15 & 1)

/**
 * Get the chunk index of a chunk descriptor.
 */
#define CHUNK_INDEX(chunkDescriptor) ((chunkDescriptor) & 8191)

// Here, "chunk width/height bit range" marks the range of bits that delimits all tile positions within the chunk,
// from 0 to the width minus 1. "chunk width/height bit index" marks the amount of bits used for that range.
// These are used to optimise division and modulo operations on chunk sizes, given that they must always be powers of 2.
//
// Here's an example, for 32 tile wide chunks:
// chunk width bit range = 31, or 11111 in binary (5 bits)
// chunk width bit index = 5

struct TileMap {
  TileMapResource* res;
  uint16_t* chunkArr;
  uint16_t* layout;
  TilePlane plane;

  uint32_t lastX;
  uint32_t lastY;

  uint32_t chunkWidth;
  uint32_t chunkWidthBitRange;
  uint32_t chunkWidthBitIndex;

  uint32_t chunkHeight;
  uint32_t chunkHeightBitRange;
  uint32_t chunkHeightBitIndex;
};

/**
 * Defines a tilemap to be used with MAP functions. For transparency's sake, this struct's members are completely
 * public and writeable, however, refrain from assigning values to them as it could cause incorrect behaviour. Unless
 * you're looking for incorrect behaviour, in which case, go ahead!
 */
typedef struct TileMap TileMap;

void MAP_loadTileMap(TileMap* map, TileMapResource* res, TilePlane plane);
void MAP_unloadTileMap(TileMap* map);
uint16_t MAP_chunkAtTile(TileMap* map, uint32_t x, uint32_t y);
uint16_t MAP_chunkAtPixel(TileMap* map, uint32_t x, uint32_t y);
void MAP_scrollTo(TileMap* map, uint32_t x, uint32_t y);