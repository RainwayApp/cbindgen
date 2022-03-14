#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Constants shared by multiple CSS Box Alignment properties
 *
 * These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
 */
typedef enum StyleAlignFlags {
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
} StyleAlignFlags;

void root(enum StyleAlignFlags flags);
