#include "andes_graphics_tiles.h"
#include "andes_res_types.h"

typedef struct TileMap TileMap;

TileMap MAP_loadTileMap(TileMapResource* res, TilePlane plane);
void MAP_unloadTileMap(TileMap* map);
void MAP_scrollTo(TileMap* map, uint32_t x, uint32_t y);
void MAP_noRefreshScrollTo(TileMap* map, uint32_t x, uint32_t y);