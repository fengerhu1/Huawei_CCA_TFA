/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */

#include <platform.h>

TARGET(binary)

OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH(aarch64)
SECTIONS
{
	. = RMM_VIRT;

	.text : {
		rmm_text_start = .;
                *(.head.text)
                . = ALIGN(8);
                *(.text)
        }

	. = ALIGN(GRANULE_SIZE);
	rmm_text_end = .;

        .rodata : {
		rmm_ro_start = .;
                *(.rodata)
        }

	. = ALIGN(GRANULE_SIZE);
	rmm_ro_end = .;

	. = ALIGN(SZ_2M);
	rmm_rw_start = .;
	.percpu : {
		. = ALIGN(GRANULE_SIZE);
		stack_start = .;
		. = . + (GRANULE_SIZE * MAX_CPUS);
		stack_end = .;
	}

        .data : {
                *(.data)
        }

	. = ALIGN(GRANULE_SIZE);

        .bss : {
                *(.bss)
        }
	. = ALIGN(SZ_2M);
	rmm_rw_end = .;

        /DISCARD/ : { *(.dynstr*) }
        /DISCARD/ : { *(.dynamic*) }
        /DISCARD/ : { *(.plt*) }
        /DISCARD/ : { *(.interp*) }
        /DISCARD/ : { *(.gnu*) }
        /DISCARD/ : { *(.note*) }
}