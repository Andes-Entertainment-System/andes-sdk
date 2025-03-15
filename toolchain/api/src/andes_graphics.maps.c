#include <stdlib.h>

#include "andes_graphics_maps.h"

struct TileMap {
  TileMapResource* res;
  uint8_t* tileSetArr;
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

  return map;
}