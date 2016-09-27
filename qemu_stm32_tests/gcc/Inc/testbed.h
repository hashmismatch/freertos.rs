/*
 * testbed.h
 *
 *  Created on: 5. sep. 2016
 *      Author: rudi
 */

#ifndef SRC_TESTBED_H_
#define SRC_TESTBED_H_

/* external */
void testbed_init();
int8_t testbed_main();

void testbed_println(uint8_t* line, uint16_t line_len);
void testbed_start_kernel();
void testbed_return(int8_t return_code);
uint32_t testbed_allocated_memory();

#endif /* SRC_TESTBED_H_ */
