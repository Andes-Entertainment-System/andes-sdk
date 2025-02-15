#pragma once

#include <stdbool.h>
#include <stdint.h>

struct PaletteResource {
  const uint8_t (*data)[3];
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
  const struct SpriteSetFrame* frames;
};

struct SoundResource {
  uint64_t address;
  uint64_t size;
  void* data;
  uint64_t dataSize;
};

struct MusicResource {
  uint64_t address;
  uint64_t size;
  float loopPoint;
};