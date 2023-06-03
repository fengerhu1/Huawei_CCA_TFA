/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */
#include <arm.h>
#include <assembler.h>

	.section ".text"

	.macro ventry_unused error_message
	.balign	0x80
	wfe
	b	.-4
	.endm

	.macro ventry label
		.balign	0x80
		b	\label
	.endm

	// VBAR_EL3[10:0] are hardwired to 0, align vector address accordingly
	.balign 0x800

ENTRY(el2_vectors):
	ventry_unused	exc_sync_sp0
	ventry_unused	exc_irq_sp0
	ventry_unused	exc_fiq_sp0
	ventry_unused	exc_serror_sp0

	ventry		el2_sync_cel
	ventry_unused	exc_irq_spx
	ventry_unused	exc_fiq_spx
	ventry_unused	exc_serror_spx

	ventry		el2_sync_lel
	ventry		el2_irq_lel
	ventry		el2_fiq_lel
	ventry_unused	exc_serror_lel

	ventry_unused	exc_sync_lel_32
	ventry_unused	exc_irq_lel_32
	ventry_unused	exc_fiq_lel_32
	ventry_unused	exc_serror_lel_32
ENDPROC(el2_vectors)

el2_sync_lel:
	stp	x0, x1, [sp, #-16]!
	mov	x0, #ARM_EXCEPTION_SYNC_LEL
	b	realm_exit
ENDPROC(el2_sync_lel)

el2_irq_lel:
	stp	x0, x1, [sp, #-16]!
	mov	x0, #ARM_EXCEPTION_IRQ_LEL
	b	realm_exit
ENDPROC(el2_sync_lel)

el2_fiq_lel:
	stp	x0, x1, [sp, #-16]!
	mov	x0, #ARM_EXCEPTION_FIQ_LEL
	b	realm_exit
ENDPROC(el2_sync_lel)

el2_sync_cel:
	stp	x2, x3, [sp, #-16]!
	stp	x0, x1, [sp, #-16]!
	mov	x0, sp
	b	handle_rmm_trap
ENDPROC(el2_sync_cel)
