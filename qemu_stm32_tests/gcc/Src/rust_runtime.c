#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "stm32f4xx_hal.h"

/* Clear memory.  Can't alias to bzero because it's not defined in the
   same translation unit.  */
void
__aeabi_memclr (void *dest, size_t n)
{
  memset (dest, 0, n);
}

void
__aeabi_memclr4 (void *dest, size_t n)
{
  memset (dest, 0, n);
}

void
__aeabi_memclr8 (void *dest, size_t n)
{
  memset (dest, 0, n);
}




/* Set memory like memset, but different argument order and no return
   value required.  */
void __aeabi_memset (void *dest, size_t n, int c)
{
  memset (dest, c, n);
}
void __aeabi_memset4(void *dest, size_t n, int c) {
	memset (dest, c, n);
}
void __aeabi_memset8(void *dest, size_t n, int c) {
	memset (dest, c, n);
}


/* Copy memory like memmove, but no return value required.  Can't
   alias to memmove because it's not defined in the same translation
   unit.  */
void
__aeabi_memmove (void *dest, const void *src, size_t n)
{
  memmove (dest, src, n);
}
void __aeabi_memmove4(void *dest, const void *src, size_t n) {
	memmove (dest, src, n);
}
