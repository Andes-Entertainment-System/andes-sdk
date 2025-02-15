#pragma once

#include <andes_graphics_palette.h>
#include <andes_graphics_sprites.h>
#include <andes_graphics_tiles.h>
#include <stdint.h>

extern void GFX_drawLine(uint8_t color, int32_t x1, int32_t y1, int32_t x2, int32_t y2);
extern void GFX_drawRect(uint8_t color, int32_t x, int32_t y, uint32_t width, uint32_t height);

void GFX_render();

extern void I_GFX_internalRender();