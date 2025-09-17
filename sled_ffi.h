#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class SledResult {
  Success,
  Error,
};

struct SledDb;

struct SledData {
  const uint8_t *ptr;
  uintptr_t len;
};

extern "C" {

SledDb *sled_open(const char *path);

void sled_close(SledDb *db_ptr);

SledResult sled_insert(SledDb *db_ptr,
                       const uint8_t *key_ptr,
                       uintptr_t key_len,
                       const uint8_t *val_ptr,
                       uintptr_t val_len);

SledData sled_get(SledDb *db_ptr, const uint8_t *key_ptr, uintptr_t key_len);

void sled_free_data(SledData data);

SledResult sled_remove(SledDb *db_ptr, const uint8_t *key_ptr, uintptr_t key_len);

}  // extern "C"
