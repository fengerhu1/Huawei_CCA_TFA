/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */
#include <mmio.h>
#include <platform.h>

#define UART_TXFE (1 << 7)

#define UART0_DR	0
#define UART0_FR	0x18

static inline void *uart_reg(unsigned long offset)
{
	return (void *)((UART0_VIRT) + offset);
}

// Wait until UART0 is not busy
static inline void UART0_wait()
{
	while (!(read8(uart_reg(UART0_FR)) & UART_TXFE));
}

// Serial output - called from printf
void _putchar(char ch)
{
	UART0_wait();

	if (ch != '\n') {
		write8(ch, uart_reg(UART0_DR));
	} else {
		write8('\r', uart_reg(UART0_DR));

		UART0_wait();

		write8('\n', uart_reg(UART0_DR));
	}
}
