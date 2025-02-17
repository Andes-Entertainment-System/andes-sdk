#pragma once
#include <andes_res_types.h>

void PAL_loadPalette(PaletteResource* res, uint8_t offset);
void PAL_loadPaletteRegion(PaletteResource* res, uint8_t offset, uint32_t regionStart, uint32_t regionSize);