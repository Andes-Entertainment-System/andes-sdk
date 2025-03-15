#include "andes_graphics_maps.h"

#include <stdlib.h>

#include "andes_storage.h"

struct TileMap {
  TileMapResource* res;
  uint16_t* chunkArr;
  uint16_t* layout;
  uint32_t lastX;
  uint32_t lastY;
  TilePlane plane;
};

TileMap MAP_loadTileMap(TileMapResource* res, TilePlane plane) {
  TileMap map;

  map.res = res;
  map.plane = plane;
  map.layout = malloc(res->layoutSize);
  map.chunkArr = malloc(res->chunkArrSize);

  STO_copyDiskToPtr(map.layout, res->layoutAddress, res->layoutSize);
  STO_copyDiskToPtr(map.chunkArr, res->chunkArrAddress, res->chunkArrSize);

  return map;
}