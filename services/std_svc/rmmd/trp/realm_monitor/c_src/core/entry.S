/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */
#include <assembler.h>

.section ".text"

ENTRY(rmm_handler):
	stp	xzr, lr, [sp, #-16]!
	bl	handle_ns_smc
	ldp	xzr, lr, [sp], #16
	smc	#0
ENDPROC(rmm_handler)
