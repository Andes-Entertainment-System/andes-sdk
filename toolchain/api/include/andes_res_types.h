#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

struct PaletteResource {
  const uint8_t (*data)[3];
  uint64_t size;
};
typedef struct PaletteResource PaletteResource;

struct RawDataResource {
  uint64_t address;
  uint64_t size;
};
typedef struct RawDataResource RawDataResource;

struct TileSetResource {
  uint64_t address;
  uint64_t size;
  uint16_t* planeArrangement;
};
typedef struct TileSetResource TileSetResource;

struct TileMapResource {
  uint64_t layoutAddress;
  uint64_t layoutSize;
  uint32_t layoutWidth;
  uint32_t layoutHeight;
  uint64_t chunkArrAddress;
  uint64_t chunkArrSize;
  uint32_t chunkWidth;
  uint32_t chunkHeight;
  TileSetResource* tileSet;
};
typedef struct TileMapResource TileMapResource;

struct SpriteSetFrame {
  uint32_t offset;
  uint32_t width;
  uint32_t height;
};
typedef struct SpriteSetFrame SpriteSetFrame;

struct SpriteSetResource {
  uint64_t address;
  uint64_t size;
  void* data;
  const SpriteSetFrame* frames;
};
typedef struct SpriteSetResource SpriteSetResource;

struct AudioResource {
  uint64_t address;
  uint64_t size;
  void* data;
  uint32_t dataSize;
  int64_t loopStart;
};
typedef struct AudioResource AudioResource;