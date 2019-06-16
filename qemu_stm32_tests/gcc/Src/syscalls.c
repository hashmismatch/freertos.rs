
/* Includes */
#include <sys/stat.h>
#include <stdlib.h>
#include <errno.h>
#include <stdio.h>
#include <signal.h>
#include <time.h>
#include <sys/time.h>
#include <sys/times.h>

#include "stm32f4xx_hal.h"
#include "semihosting.h"

/* Variables */
#undef errno
extern int errno;
//extern int __io_putchar(int ch) __attribute__((weak));
extern int __io_getchar(void) __attribute__((weak));

extern void usart2_send_byte(uint8_t byte);

static int __io_putchar(int ch) {
	return trace_write(ch);
}

/*
static int __sfputc_r(int ch) {
	UartWriteChar(ch);
}
*/
register char * stack_ptr __asm("sp");

char *__env[1] = { 0 };
char **environ = __env;


/* Functions */
void initialise_monitor_handles()
{
}

int _getpid(void)
{
	return 1;
}

int _kill(int pid, int sig)
{
	errno = EINVAL;
	return -1;
}

void _exit (int status)
{
	_kill(status, -1);
	while (1) {}		/* Make sure we hang here */
}

int _read (int file, char *ptr, int len)
{
	int DataIdx;

	for (DataIdx = 0; DataIdx < len; DataIdx++)
	{
	  *ptr++ = __io_getchar();
	}

return len;
}

int _write(int file, char *ptr, int len)
{
	int DataIdx;

	for (DataIdx = 0; DataIdx < len; DataIdx++)
	{
	   __io_putchar( *ptr++ );
	}
	return len;
}

static char *heap_end;
static char *heap_start;
caddr_t _sbrk(int incr)
{
	extern char end __asm("end");
	char *prev_heap_end;

	if (heap_end == 0) {
		heap_end = &end;
		heap_start = &end;
	}

	prev_heap_end = heap_end;
	/*
	if (heap_end + incr > stack_ptr)
	{
//		write(1, "Heap and stack collision\n", 25);
//		abort();
		errno = ENOMEM;
		return (caddr_t) -1;
	}
	*/

	heap_end += incr;

	return (caddr_t) prev_heap_end;
}

uint32_t heap_bytes_used() {
	return heap_end - heap_start;
}

int _close(int file)
{
	return -1;
}


int _fstat(int file, struct stat *st)
{
	st->st_mode = S_IFCHR;
	return 0;
}

int _isatty(int file)
{
	return 1;
}

int _lseek(int file, int ptr, int dir)
{
	return 0;
}

int _open(char *path, int flags, ...)
{
	/* Pretend like we always fail */
	return -1;
}

int _wait(int *status)
{
	errno = ECHILD;
	return -1;
}

int _unlink(char *name)
{
	errno = ENOENT;
	return -1;
}

int _times(struct tms *buf)
{
	return -1;
}

int _stat(char *file, struct stat *st)
{
	st->st_mode = S_IFCHR;
	return 0;
}

int _link(char *old, char *new)
{
	errno = EMLINK;
	return -1;
}

int _fork(void)
{
	errno = EAGAIN;
	return -1;
}

int _execve(char *name, char **argv, char **env)
{
	errno = ENOMEM;
	return -1;
}
