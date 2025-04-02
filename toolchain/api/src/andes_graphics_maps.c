#include "andes_graphics_maps.h"

#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#include "andes_graphics.h"
#include "andes_storage.h"

#define ALIGN_PIXEL_TO_TILE(x) (x >> 3)

#define SCREEN_WIDTH_TILES_PADDED (GFX_SCREEN_WIDTH_TILES + 1)
#define SCREEN_HEIGHT_TILES_PADDED (GFX_SCREEN_HEIGHT_TILES + 1)

void MAP_loadTileMap(TileMap* map, TileMapResource* res, TilePlane plane) {
  map->res = res;
  map->plane = plane;
  map->layout = malloc(res->layoutSize);
  map->chunkArr = malloc(res->chunkArrSize);

  map->lastX = INT32_MAX;
  map->lastY = INT32_MAX;

  map->chunkWidth = map->res->chunkWidth;
  map->chunkWidthBitRange = map->chunkWidth - 1;
  map->chunkWidthBitIndex = sqrt(map->chunkWidth);
  map->chunkHeight = map->res->chunkHeight;
  map->chunkHeightBitRange = map->chunkHeight - 1;
  map->chunkHeightBitIndex = sqrt(map->chunkHeight);

  STO_copyDiskToPtr(map->layout, res->layoutAddress, res->layoutSize);
  STO_copyDiskToPtr(map->chunkArr, res->chunkArrAddress, res->chunkArrSize);
}

void MAP_unloadTileMap(TileMap* map) {
  free(map->layout);
  free(map->chunkArr);

  map->layout = NULL;
  map->chunkArr = NULL;
}

uint16_t MAP_chunkAt(TileMap* map, uint32_t x, uint32_t y) {
  return map->layout[(x >> map->chunkWidthBitIndex) + (y >> map->chunkHeightBitIndex) * map->res->layoutWidth];
}

void I_MAP_loadTiles(TileMap* map, uint32_t x, uint32_t y, uint32_t width, uint32_t height) {
  uint16_t tileRect[height][width];

  uint16_t chunkDescriptor;

  uint32_t chunkX = UINT32_MAX;
  uint32_t chunkY = UINT32_MAX;

  uint16_t* chunkArr;

  bool chunkHFlip, chunkVFlip, chunkPriority;

  for (uint32_t ty = 0; ty < height; ty++) {
    for (uint32_t tx = 0; tx < width; tx++) {
      uint32_t mapX = tx + x;
      uint32_t mapY = ty + y;

      if (mapX >> map->chunkWidthBitIndex != chunkX || mapY >> map->chunkHeightBitIndex != chunkY) {
        chunkX = mapX >> map->chunkWidthBitIndex;
        chunkY = mapY >> map->chunkHeightBitIndex;

        chunkDescriptor = map->layout[chunkX + chunkY * map->res->layoutWidth];
        chunkHFlip = CHUNK_HFLIP_FLAG(chunkDescriptor);
        chunkVFlip = CHUNK_VFLIP_FLAG(chunkDescriptor);
        chunkPriority = CHUNK_PRIORITY_FLAG(chunkDescriptor);
        chunkArr = map->chunkArr + CHUNK_INDEX(chunkDescriptor) * map->chunkWidth * map->chunkHeight;
      }

      uint32_t tileX = (mapX & map->chunkWidthBitRange);
      uint32_t tileY = (mapY & map->chunkHeightBitRange);
      if (chunkHFlip) tileX = map->chunkWidthBitRange - tileX;
      if (chunkVFlip) tileY = map->chunkHeightBitRange - tileY;

      tileRect[ty][tx] =
          TILE_DESCRIPTOR(chunkArr[tileX + tileY * map->chunkWidth], chunkPriority, chunkHFlip, chunkVFlip);
    }
  }

  TIL_setPlaneRect(map->plane, x, y, width, height, (uint16_t*)tileRect);
}

void MAP_scrollTo(TileMap* map, uint32_t x, uint32_t y) {
  uint32_t tileX = ALIGN_PIXEL_TO_TILE(x);
  uint32_t tileY = ALIGN_PIXEL_TO_TILE(y);

  int32_t deltaX = (int32_t)tileX - (int32_t)map->lastX;
  int32_t deltaY = (int32_t)tileY - (int32_t)map->lastY;

  TIL_setPlaneScrollXY(map->plane, x, y);

  // if the map is scrolled so far from the last point such that none of the tiles already drawn on the plane
  // are conserved, just load an entire "screenful" of tiles and call it a day
  if (abs(deltaX) > GFX_SCREEN_WIDTH_TILES || abs(deltaY) > GFX_SCREEN_HEIGHT_TILES) {
    I_MAP_loadTiles(map, tileX, tileY, SCREEN_WIDTH_TILES_PADDED, SCREEN_HEIGHT_TILES_PADDED);

    map->lastX = tileX;
    map->lastY = tileY;

    return;
  }

  // column loading
  if (deltaX > 0) {
    I_MAP_loadTiles(map, map->lastX + SCREEN_WIDTH_TILES_PADDED, tileY, deltaX, SCREEN_HEIGHT_TILES_PADDED);
  } else if (deltaX < 0) {
    I_MAP_loadTiles(map, tileX, tileY, -deltaX, SCREEN_HEIGHT_TILES_PADDED);
  }

  // row loading
  if (deltaY > 0) {
    I_MAP_loadTiles(map, tileX, map->lastY + SCREEN_HEIGHT_TILES_PADDED, SCREEN_WIDTH_TILES_PADDED, deltaY);
  } else if (deltaY < 0) {
    I_MAP_loadTiles(map, tileX, tileY, SCREEN_WIDTH_TILES_PADDED, -deltaY);
  }

  map->lastX = tileX;
  map->lastY = tileY;
}