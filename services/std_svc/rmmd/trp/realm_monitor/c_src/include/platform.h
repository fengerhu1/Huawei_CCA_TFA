
/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */
#ifndef __PLATFORM_H_
#define __PLATFORM_H_

#include <const.h>
#include <sizes.h>

#define RMM_PHYS	UL(0x0000000006000000)
#define RMM_VIRT	UL(0xfffffff000000000)
#define SLOT_VIRT	UL(0xffffffff80000000) // (1 << 64) - (2 << 30)

#define UART0_VIRT	UL(0xffffffffffff0000)
#define UART0_PHYS	UL(0x001c0a0000)

#define MEM0_PHYS	UL(0x0000000080000000)
#define MEM0_SIZE	SZ_2G
#define ASC0_VIRT	UL(0xfffffffffffe0000)

#define MEM1_PHYS	UL(0x0000000880000000)
#define MEM1_SIZE	SZ_2G

/*
 * This RMM implementaiton supports at most 16 CPUS, which can be indexed by
 * Aff0 alone.
 */
#define MAX_CPUS	16

/*
 * The RMM is mapped with 4K pages, and all RMM APIs use the same granularity.
 */
#define GRANULE_SIZE	SZ_4K
#define ALIGNED(_size, _alignment) (((unsigned long)(_size) % (_alignment)) == 0)
#define GRANULE_ALIGNED(_addr) ALIGNED(_addr, GRANULE_SIZE)
#define GRANULE_SHIFT	(12)
#define GRANULE_MASK	(~0xfffUL)

#define PGTES_PER_TABLE (1 << (GRANULE_SHIFT - 3))
#define BLOCK_L2_SIZE (GRANULE_SIZE * PGTES_PER_TABLE)

#define NR_GRANULES	(MEM0_SIZE / GRANULE_SIZE)

#define HAS_MPAM 0

#if HAS_MPAM
#define MPAM(_x...) _x
#else
#define MPAM(_x...)
#endif

#define HAS_SVE 0

#if HAS_SVE
#define SVE(_x...) _x
#else
#define SVE(_x...)
#endif


#endif /* __PLATFORM_H_ */
