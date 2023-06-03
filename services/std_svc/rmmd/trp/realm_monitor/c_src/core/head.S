/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */
#include <assembler.h>
#include <sizes.h>
#include <sysreg.h>
#include <platform.h>
#include <mmu.h>

.section ".head.text", "ax"

rmm_entry:
	// This is the main entry point for the RMM
	// ----------------------------------------
	cpuid	x1

	/* Allocate per cpu stacks
	 * stack-start			     stack_end
	 *      o--4K---o--4K---o--4K---o--4K---o
	 *      ^\_____/^\_____/^\_____/^\_____/^
	 *       id = 3     2       1       0
	 */
	va_to_pa x0, stack_end
	mov	x2, #SZ_4K	  // 4K stack size per CPU
	umsubl	x0, w1, w2, x0
	mov	sp, x0

	/* x0: cpuid */
	mov	x0, x1
	/* Create pagetable entries for rmm and  switch on the mmu.
	 * The stack pointer will be corrected to the right va address
	 */
	bl	setup_mmu

	/* MMU is switched on and only the upper half of the translation is
	 * used for RMM. Switch off the lower half of the translation.
	 */
	mrs	x0, tcr_el2
	orr	x0, x0, #TCR_EL2_EPD0_BIT
	msr	tcr_el2, x0
	isb
	tlbi	alle2is
	dsb	sy

	ldr	x0, =el2_vectors
	msr	vbar_el2, x0

	bl rmm_setup
	mov	x0, xzr
	smc	#0
