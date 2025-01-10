#pragma once
#include <andes_res_types.h>

enum TilePlane {
  TILEPLANE_BG,
  TILEPLANE_FG,
};

void TIL_loadTileSet(struct TileSetResource* res);
void TIL_loadTileSetAt(struct TileSetResource* res, uint32_t address);

extern void TIL_setPlaneScrollXY(enum TilePlane plane, uint32_t x, uint32_t y);

/**
 * @brief In the specified tile plane, set the tile located at (`x`, `y`) to the value of `tile`.
 */
extern void TIL_setPlaneXY(enum TilePlane plane, uint32_t x, uint32_t y, uint16_t tile);
/**
 * @brief In the specified tile plane, starting from the tile located at (`x`, `y`) and going down, set `n` amount of
 *        tiles to the values in `tiles`.
 */
extern void TIL_setPlaneColumn(enum TilePlane plane, uint32_t x, uint32_t y, uint16_t* tiles, uint32_t n);
/**
 * @brief In the specified tile plane, starting from the tile located at (`x`, `y`) and going right, set `n` amount of
 *        tiles to the values in `tiles`.
 */
extern void TIL_setPlaneRow(enum TilePlane plane, uint32_t x, uint32_t y, uint16_t* tiles, uint32_t n);
/**
 * @brief In the specified tile plane, copy the values in `tiles` into a rectangle denoted by (`x`, `y`, `width`,
 *        `height`), going left-to-right and top-to-bottom.
 */
extern void TIL_setPlaneRect(enum TilePlane plane, uint32_t x, uint32_t y, uint32_t width, uint32_t height,
                             uint16_t* tiles);