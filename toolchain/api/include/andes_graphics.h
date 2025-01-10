#pragma once

#include <andes_graphics_palette.h>
#include <andes_graphics_sprites.h>
#include <andes_graphics_tiles.h>
#include <stdint.h>

extern void GFX_drawLine(uint8_t color, uint32_t x1, uint32_t y1, uint32_t x2, uint32_t y2);
extern void GFX_drawRect(uint8_t color, uint32_t x, uint32_t y, uint32_t width, uint32_t height);

void GFX_render();

extern void I_GFX_internalRender();