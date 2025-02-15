#include <andes_graphics_palette.h>
#include <andes_storage.h>

void PAL_loadPalette(struct PaletteResource* res, uint8_t offset) {
  STO_copyPtrToRegister(REG_PALETTE, offset * 3, (void*)res->data, res->size);
}

void PAL_loadPaletteRegion(struct PaletteResource* res, uint8_t offset, uint32_t regionStart, uint32_t regionSize) {
  STO_copyPtrToRegister(REG_PALETTE, offset * 3, (void*)(res->data + regionStart), regionSize * 3);
}