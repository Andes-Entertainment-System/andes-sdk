#pragma once
#include <andes_res_types.h>

/**
 * Assemble a tile descriptor for a plane, using the specified tile index, priority and flip flags.
 */
#define TILE_DESCRIPTOR(tileIndex, priority, hFlip, vFlip) \
  ((tileIndex) | (priority) << 15 | (vFlip) << 14 | (hFlip) << 13)

/**
 * Get the horizontal flip flag of a tile descriptor.
 */
#define TILE_HFLIP_FLAG(tileDescriptor) ((tileDescriptor) >> 13 & 1)

/**
 * Get the vertical flip flag of a tile descriptor.
 */
#define TILE_VFLIP_FLAG(tileDescriptor) ((tileDescriptor) >> 14 & 1)

/**
 * Get the priority flag of a tile descriptor.
 */
#define TILE_PRIORITY_FLAG(tileDescriptor) ((tileDescriptor) >> 15)

/**
 * Get the tile index of a tile descriptor.
 */
#define TILE_INDEX(tileDescriptor) ((tileDescriptor) & 8191)

#define TILE_PLANE_WIDTH 64
#define TILE_PLANE_HEIGHT 64

enum TilePlane {
  TILEPLANE_BG,
  TILEPLANE_FG,
};
typedef enum TilePlane TilePlane;

void TIL_loadTileSet(TileSetResource* res);
void TIL_loadTileSetAt(TileSetResource* res, uint32_t offset);

/**
 * @brief In the specified tile plane, starting from the row of tiles located at `row` and going down, set `n` amount of
 *        values from the plane's horizontal scroll offset table to the values in `offsets`.
 */
void TIL_setPlaneHScrollTable(TilePlane plane, uint32_t row, uint32_t* offsets, uint32_t n);
/**
 * @brief Set `x` (horizontal) and `y` (vertical) base scroll values for the specified tile plane.
 */
extern void TIL_setPlaneScrollXY(TilePlane plane, uint32_t x, uint32_t y);

/**
 * @brief In the specified tile plane, set the tile located at (`x`, `y`) to the value of `tile`.
 */
extern void TIL_setPlaneSingle(TilePlane plane, uint32_t x, uint32_t y, uint16_t tile);
/**
 * @brief In the specified tile plane, starting from the tile located at (`x`, `y`) and going down, set `n` amount of
 *        tiles to the values in `tiles`.
 */
extern void TIL_setPlaneColumn(TilePlane plane, uint32_t x, uint32_t y, uint16_t* tiles, uint32_t n);
/**
 * @brief In the specified tile plane, starting from the tile located at (`x`, `y`) and going right, set `n` amount of
 *        tiles to the values in `tiles`.
 */
extern void TIL_setPlaneRow(TilePlane plane, uint32_t x, uint32_t y, uint16_t* tiles, uint32_t n);
/**
 * @brief In the specified tile plane, copy the values in `tiles` into a rectangle denoted by (`x`, `y`, `width`,
 *        `height`), going left-to-right and top-to-bottom.
 */
extern void TIL_setPlaneRect(TilePlane plane, uint32_t x, uint32_t y, uint32_t width, uint32_t height, uint16_t* tiles);

/**
 * @brief In the specified tile plane, starting from the tile located at (`x`, `y`) and going down, set `n` amount of
 *        tiles to the value of `tile`.
 */
extern void TIL_fillPlaneColumn(TilePlane plane, uint32_t x, uint32_t y, uint16_t tile, uint32_t n);
/**
 * @brief In the specified tile plane, starting from the tile located at (`x`, `y`) and going right, set `n` amount of
 *        tiles to the value of `tile`.
 */
extern void TIL_fillPlaneRow(TilePlane plane, uint32_t x, uint32_t y, uint16_t tile, uint32_t n);
/**
 * @brief In the specified tile plane, fill a rectangle denoted by (`x`, `y`, `width`,
 *        `height`), going left-to-right and top-to-bottom, to the value of `tile`.
 */
extern void TIL_fillPlaneRect(TilePlane plane, uint32_t x, uint32_t y, uint32_t width, uint32_t height, uint16_t tile);