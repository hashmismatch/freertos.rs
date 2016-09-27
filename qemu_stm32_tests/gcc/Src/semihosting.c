#include "semihosting.h"

void
semihost_exit (int status)
{
  /* There is only one SWI for both _exit and _kill. For _exit, call
   the SWI with the second argument set to -1, an invalid value for
   signum, so that the SWI handler can distinguish the two calls.
   Note: The RDI implementation of _kill throws away both its
   arguments.  */
  report_exception (
      status == 0 ? ADP_Stopped_ApplicationExit : ADP_Stopped_RunTimeError);
}

uint8_t trace_write(uint8_t b) {
	ITM_SendChar(b);
	return 1;
}

uint16_t trace_print(char* str, uint16_t len) {
	int i = 0;
	while (len-- > 0) {
		trace_write(str[i++]);
	}
	return i;
}
