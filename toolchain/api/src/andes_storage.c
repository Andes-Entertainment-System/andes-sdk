#include <andes_storage.h>

void STO_copyRawDataToPtr(void* dest, struct RawDataResource* src, uint64_t srcAddr, uint64_t n) {
  STO_copyDiskToPtr(dest, src->address + srcAddr, n == 0 ? src->size : n);
}
