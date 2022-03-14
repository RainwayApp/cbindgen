#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Constants shared by multiple CSS Box Alignment properties
 *
 * These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
 */
typedef enum AlignFlags {
  /**
   * 'auto'
   */
  AUTO = 0,
  /**
   * 'normal'
   */
  NORMAL = 1,
  /**
   * 'start'
   */
  START = (1 << 1),
  /**
   * 'end'
   */
  END = (1 << 2),
  /**
   * 'flex-start'
   */
  FLEX_START = (1 << 3),
} AlignFlags;

typedef enum DebugFlags {
  /**
   * Flag with the topmost bit set of the u32
   */
  BIGGEST_ALLOWED = (1 << 31),
} DebugFlags;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(enum AlignFlags flags, enum DebugFlags bigger_flags);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
