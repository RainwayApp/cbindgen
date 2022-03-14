from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # Constants shared by multiple CSS Box Alignment properties
  #
  # These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
  ctypedef enum StyleAlignFlags:
    # 'auto'
    AUTO # = 0,
    # 'normal'
    NORMAL # = 1,
    # 'start'
    START # = (1 << 1),
    # 'end'
    END # = (1 << 2),
    # 'flex-start'
    FLEX_START # = (1 << 3),

  void root(StyleAlignFlags flags);
