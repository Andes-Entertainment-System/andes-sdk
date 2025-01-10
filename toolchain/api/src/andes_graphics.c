#include <andes_graphics.h>
#include <andes_graphics_sprites.h>

void GFX_render() {
  I_SPR_onRender();
  I_GFX_internalRender();
}