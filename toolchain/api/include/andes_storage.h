#pragma once
#include <andes_res_types.h>

enum AndesRegister {
  REG_PALETTE,
  REG_TILEMAP,
  REG_BG_PLANE,
  REG_FG_PLANE,
  REG_FRAME_BUFFER,
  REG_GAMEPAD_STATES,
};

extern void STO_copyPtrToRegister(enum AndesRegister dest, uint32_t destAddr, void* src, uint32_t n);
extern void STO_copyDiskToRegister(enum AndesRegister dest, uint32_t destAddr, uint64_t srcAddr, uint32_t n);
extern void STO_copyRegisterToPtr(void* dest, enum AndesRegister src, uint32_t srcAddr, uint32_t n);
extern void STO_copyDiskToPtr(void* dest, uint64_t srcAddr, uint32_t n);

void STO_copyRawDataToPtr(void* dest, struct RawDataResource* src, uint64_t srcAddr, uint64_t n);
