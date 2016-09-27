/*
FreeRTOS.rs shim library

Include headers relevant for your platform.

STM32 example:

#include "stm32f4xx_hal.h"

*/

#include "FreeRTOS.h"

uint8_t freertos_rs_sizeof(uint8_t _type) {
	switch (_type) {
		case 0:
			return sizeof(void*);
			break;
		case 1:
			return sizeof(char*);
			break;
		case 2:
			return sizeof(char);
			break;
		
		case 10:
			return sizeof(BaseType_t);
			break;
		case 11:
			return sizeof(UBaseType_t);
			break;
		case 12:
			return sizeof(TickType_t);
			break;		
		
		case 20:
			return sizeof(TaskHandle_t);
			break;
		case 21:
			return sizeof(QueueHandle_t);
			break;
		case 22:
			return sizeof(SemaphoreHandle_t);
			break;
		case 23:
			return sizeof(TaskFunction_t);
			break;

		break;
		default:
			return 0;
	}
}

#if (INCLUDE_vTaskDelayUntil == 1)
void freertos_rs_vTaskDelayUntil(TickType_t *pxPreviousWakeTime, TickType_t xTimeIncrement) {
	vTaskDelayUntil(pxPreviousWakeTime, xTimeIncrement);
}
#endif

#if (INCLUDE_vTaskDelay == 1)
void freertos_rs_vTaskDelay(TickType_t xTicksToDelay) {
	vTaskDelay(xTicksToDelay);
}
#endif

TickType_t freertos_rs_xTaskGetTickCount() {
	return xTaskGetTickCount();
}

TickType_t freertos_rs_get_portTICK_PERIOD_MS() {
	return portTICK_PERIOD_MS;
}

#if (configUSE_RECURSIVE_MUTEXES == 1)
QueueHandle_t freertos_rs_create_recursive_mutex() {
	return xSemaphoreCreateRecursiveMutex();
}

UBaseType_t freertos_rs_take_recursive_mutex(QueueHandle_t mutex, UBaseType_t max) {
	if (xSemaphoreTakeRecursive(mutex, max) == pdTRUE) {
		return 0;
	}

	return 1;
}
UBaseType_t freertos_rs_give_recursive_mutex(QueueHandle_t mutex) {
	if (xSemaphoreGiveRecursive(mutex) == pdTRUE) {
		return 0;
	} else {
		return 1;
	}
}
#endif

QueueHandle_t freertos_rs_create_mutex() {
	return xSemaphoreCreateMutex();
}

QueueHandle_t freertos_rs_create_binary_semaphore() {
	return xSemaphoreCreateBinary();
}

QueueHandle_t freertos_rs_create_counting_semaphore(UBaseType_t max, UBaseType_t initial) {
	return xSemaphoreCreateCounting(max, initial);
}

void freertos_rs_delete_semaphore(QueueHandle_t semaphore) {
	vSemaphoreDelete(semaphore);
}

UBaseType_t freertos_rs_take_mutex(QueueHandle_t mutex, UBaseType_t max) {
	if (xSemaphoreTake(mutex, max) == pdTRUE) {
		return 0;
	}

	return 1;
}

UBaseType_t freertos_rs_give_mutex(QueueHandle_t mutex) {
	if (xSemaphoreGive(mutex) == pdTRUE) {
		return 0;
	}

	return 1;
}

UBaseType_t freertos_rs_take_semaphore_isr(QueueHandle_t semaphore, BaseType_t* xHigherPriorityTaskWoken) {
	if (xSemaphoreTakeFromISR(semaphore, xHigherPriorityTaskWoken) == pdTRUE) {
		return 0;
	}

	return 1;
}

UBaseType_t freertos_rs_give_semaphore_isr(QueueHandle_t semaphore, BaseType_t* xHigherPriorityTaskWoken) {
	if (xSemaphoreGiveFromISR(semaphore, xHigherPriorityTaskWoken) == pdTRUE) {
		return 0;
	}

	return 1;
}


UBaseType_t freertos_rs_spawn_task(TaskFunction_t entry_point, void* pvParameters, const char * const name, uint8_t name_len, uint16_t stack_size, UBaseType_t priority, TaskHandle_t* task_handle) {
	uint8_t c_name[configMAX_TASK_NAME_LEN] = {0};
	for (int i = 0; i < name_len; i++) {
		c_name[i] = name[i];

		if (i == configMAX_TASK_NAME_LEN - 1) {
			break;
		}
	}

	BaseType_t ret = xTaskCreate(entry_point, c_name, stack_size, pvParameters, priority, task_handle);

	if (ret != pdPASS) {
		return 1;
	}

	configASSERT(task_handle);

	return 0;
}

#if (INCLUDE_vTaskDelete == 1)
void freertos_rs_delete_task(TaskHandle_t task) {
	vTaskDelete(task);
}
#endif


QueueHandle_t freertos_rs_queue_create(UBaseType_t queue_length, UBaseType_t item_size) {
	return xQueueCreate(queue_length, item_size);
}

void freertos_rs_queue_delete(UBaseType_t queue) {
	vQueueDelete(queue);
}

UBaseType_t freertos_rs_queue_send(QueueHandle_t queue, void* item, TickType_t max_wait) {
    if (xQueueSend(queue, item, max_wait ) != pdTRUE)
    {
        return 1;
    }

    return 0;
}

UBaseType_t freertos_rs_queue_send_isr(QueueHandle_t queue, void* item, BaseType_t* xHigherPriorityTaskWoken) {
	UBaseType_t ret = 1;
	
	if (xQueueSendFromISR(queue, item, xHigherPriorityTaskWoken) == pdTRUE) {
		return 0;
	}
	return 1;
}

UBaseType_t freertos_rs_queue_receive(QueueHandle_t queue, void* item, TickType_t max_wait) {
	if ( xQueueReceive( queue, item, max_wait ) != pdTRUE )
	{
		return 1;
	}

	return 0;
}

void freertos_rs_isr_yield() {
	portYIELD();
}

TickType_t freertos_rs_max_wait() {
	return portMAX_DELAY;
}

#if (INCLUDE_pcTaskGetTaskName == 1)
char* freertos_rs_task_get_name(TaskHandle_t task) {
	return pcTaskGetTaskName(task);
}
#endif

uint32_t freertos_rs_task_notify_take(uint8_t clear_count, TickType_t wait) {
	return ulTaskNotifyTake(clear_count == 1 ? pdTRUE : pdFALSE, wait);
}

BaseType_t freertos_rs_task_notify_wait(uint32_t ulBitsToClearOnEntry, uint32_t ulBitsToClearOnExit, uint32_t *pulNotificationValue, TickType_t xTicksToWait) {
	if (xTaskNotifyWait(ulBitsToClearOnEntry, ulBitsToClearOnExit, pulNotificationValue, xTicksToWait) == pdTRUE) {
		return 0;
	}

	return 1;
}

eNotifyAction freertos_rs_task_notify_action(uint8_t action) {
	switch (action) {					
		case 1:
			return eSetBits;
		case 2:
			return eIncrement;
		case 3:
			return eSetValueWithOverwrite;
		case 4:
			return eSetValueWithoutOverwrite;
		default:
			return eNoAction;
	}
}

BaseType_t freertos_rs_task_notify(void* task, uint32_t value, uint8_t action) {
	eNotifyAction eAction = freertos_rs_task_notify_action(action);

	BaseType_t v = xTaskNotify(task, value, eAction);
	if (v != pdPASS) { 
		return 1;
	}
	return 0;
}

BaseType_t freertos_rs_task_notify_isr(void* task, uint32_t value, uint8_t action, BaseType_t* xHigherPriorityTaskWoken) {
	eNotifyAction eAction = freertos_rs_task_notify_action(action);

	BaseType_t v = xTaskNotifyFromISR(task, value, eAction, xHigherPriorityTaskWoken);
	if (v != pdPASS) { 
		return 1;
	}
	return 0;
}

#if ( ( INCLUDE_xTaskGetCurrentTaskHandle == 1 ) || ( configUSE_MUTEXES == 1 ) )
TaskHandle_t freertos_rs_get_current_task() {
	return xTaskGetCurrentTaskHandle();
}
#endif
