/*
 * Copyright (c) 2013-2018, ARM Limited and Contributors. All rights reserved.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#ifndef BL_COMMON_H
#define BL_COMMON_H

#include <lib/utils_def.h>

#define UP	U(1)
#define DOWN	U(0)

/*******************************************************************************
 * Constants to identify the location of a memory region in a given memory
 * layout.
******************************************************************************/
#define TOP	U(0x1)
#define BOTTOM	U(0x0)

/*
 * Declarations of linker defined symbols to help determine memory layout of
 * BL images
 */
IMPORT_SYM(unsigned long, __TEXT_START__,	BL_CODE_BASE);
IMPORT_SYM(unsigned long, __TEXT_END__,		BL_CODE_END);
IMPORT_SYM(unsigned long, __RODATA_START__,	BL_RO_DATA_BASE);
IMPORT_SYM(unsigned long, __RODATA_END__,	BL_RO_DATA_END);

#endif /* BL_COMMON_H */
