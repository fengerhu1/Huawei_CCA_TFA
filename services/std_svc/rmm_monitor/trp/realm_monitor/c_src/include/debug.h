/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */
#ifndef __DEBUG_H_
#define __DEBUG_H_

#include <printf.h>

#ifndef DEBUG

static inline void pr_debug(const char *fmt, ...) { }

#else

static inline void pr_debug(const char *fmt, ...)
{
	va_list args;

	va_start(args, fmt);
	vprintf(fmt, args);
	va_end(args);
}

#endif

#endif /* __DEBUG_H_ */
