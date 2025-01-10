#include <andes_graphics_tiles.h>
#include <andes_storage.h>

void TIL_loadTileSet(struct TileSetResource* res) { STO_copyDiskToRegister(REG_TILEMAP, 0, res->address, res->size); }

void TIL_loadTileSetAt(struct TileSetResource* res, uint32_t address) {
  STO_copyDiskToRegister(REG_TILEMAP, address, res->address, res->size);
}