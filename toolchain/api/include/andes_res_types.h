#pragma once

#include <stdbool.h>
#include <stdint.h>

struct PaletteResource {
  uint16_t* data;
  uint64_t size;
};

struct RawDataResource {
  uint64_t address;
  uint64_t size;
};

struct TileSetResource {
  uint64_t address;
  uint64_t size;
};

struct SpriteSetFrame {
  uint32_t offset;
  uint32_t width;
  uint32_t height;
};

struct SpriteSetResource {
  uint64_t address;
  uint64_t size;
  void* data;
  struct SpriteSetFrame* frames;
};