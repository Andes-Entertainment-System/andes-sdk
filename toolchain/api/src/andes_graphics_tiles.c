#include <andes_graphics_tiles.h>
#include <andes_storage.h>

void TIL_loadTileSet(struct TileSetResource* res) { STO_copyDiskToRegister(REG_TILEMAP, 0, res->address, res->size); }

void TIL_loadTileSetAt(struct TileSetResource* res, uint32_t offset) {
  STO_copyDiskToRegister(REG_TILEMAP, offset * 64, res->address, res->size);
}

void TIL_setPlaneHScrollTable(enum TilePlane plane, uint32_t row, uint32_t* offsets, uint32_t n) {
  STO_copyPtrToRegister(plane == TILEPLANE_FG ? REG_FG_HSCROLL_TABLE : REG_BG_HSCROLL_TABLE, row, offsets,
                        n * sizeof(uint32_t));
}