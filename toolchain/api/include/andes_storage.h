#pragma once
#include <andes_res_types.h>

enum AndesRegister {
  REG_PALETTE,
  REG_TILEMAP,
  REG_BG_PLANE,
  REG_FG_PLANE,
  REG_BG_HSCROLL_TABLE,
  REG_FG_HSCROLL_TABLE,
  REG_FRAME_BUFFER,
  REG_GAMEPAD_STATES,
};
typedef enum AndesRegister AndesRegister;

extern void STO_copyPtrToRegister(AndesRegister dest, uint32_t destAddr, void* src, uint32_t n);
extern void STO_copyDiskToRegister(AndesRegister dest, uint32_t destAddr, uint64_t srcAddr, uint32_t n);
extern void STO_copyRegisterToPtr(void* dest, AndesRegister src, uint32_t srcAddr, uint32_t n);
extern void STO_copyDiskToPtr(void* dest, uint64_t srcAddr, uint32_t n);

void STO_copyRawDataToPtr(void* dest, struct RawDataResource* src, uint64_t srcAddr, uint64_t n);
