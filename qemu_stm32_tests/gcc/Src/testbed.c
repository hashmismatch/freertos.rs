#include "stm32f4xx_hal.h"
#include "semihosting.h"
#include "testbed.h"
#include "malloc.h"
#include "stm32f4xx_it.h"
#include "FreeRTOS.h"
#include "task.h"
#include "cmsis_os.h"
#include "tim.h"

static int8_t kernel_started = 0;

void testbed_println(uint8_t* line, uint16_t line_len) {	
	if (kernel_started == 1) {
		taskENTER_CRITICAL();
	}
	trace_print(line, line_len);
	trace_print("\n", 1);
	if (kernel_started == 1) {
		taskEXIT_CRITICAL();
	}
}

void testbed_init() {
	
}

void timer4_emulator(void const * argument) {
	while (1) {
		vTaskDelay(50);
		NVIC_SetPendingIRQ(TIM4_IRQn);
	}
}

void testbed_init_timer4_50ms_isr() {
	if (HAL_TIM_Base_Start_IT(&htim4) != HAL_OK) {
    	Error_Handler();
  	}  

	osThreadId handle;
	osThreadDef(timerTask, timer4_emulator, osPriorityNormal, 0, 128);
  	handle = osThreadCreate(osThread(timerTask), NULL);
}


void testbed_start_kernel() {
	kernel_started = 1;
	osKernelStart();	
}

void testbed_return(int8_t return_code) {
	semihost_exit(return_code);
}

uint32_t testbed_allocated_memory() {
	return mallinfo().uordblks;
}