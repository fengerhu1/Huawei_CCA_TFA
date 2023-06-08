#ifndef __PAGETABLE_H_
#define __PAGETABLE_H_

#include <platform.h>

#define TBL_SIZE		GRANULE_SIZE
#define DESC_SIZE		8
#define DESC_PER_TBL_MASK	((TBL_SIZE/DESC_SIZE) - 1)

#if GRANULE_SIZE == SZ_4K
	#define DESC_PER_TBL_SHIFT	(9)
	#define L3_BLK_SZ		(SZ_4K)
	#define L3_SHIFT		(GRANULE_SHIFT)
	#define L3_MASK			(DESC_PER_TBL_MASK << L3_SHIFT)
	#define L3_PAGE_ENTRY		(0b11)
	#define L2_BLK_SZ		(SZ_2M)
	#define L2_SHIFT		(L3_SHIFT + DESC_PER_TBL_SHIFT)
	#define L2_MASK			(DESC_PER_TBL_MASK << L2_SHIFT)
	#define L2_BLOCK_ENTRY		(0b01)
#else
#error "Unsupported granule size"
#endif


/* table attributes */
#define NS_TABLE		(0x1ULL << 63)
#define AP1_TABLE		(0x1ULL << 62)
#define AP0_TABLE		(0x1ULL << 61)
#define XN_TABLE		(0x1ULL << 60)
#define PXN_TABLE		(0x1ULL << 69)

/* page/block attributes */
#define XN			(0x1ULL << 54)
#define PXN			(0x1ULL << 53)

#define NG			(0x1ULL << 11)
#define AF			(0x1ULL << 10)

#define SH(_sh)			(((_sh) & 0x3) << 8)
#define NON_SHAREABLE		0x0
#define OUTER_SHAREABLE		0x2
#define INNER_SHAREABLE		0x3

#define AP(_ap)			(((_ap) & 0x3) << 6)
#define RW			0x0
#define RO			0x2

#define NS			(0x1ULL << 5)
#define ATTRIDX(_idx)		(((_idx) & 0x7) << 2)

#define SH(_sh)			(((_sh) & 0x3) << 8)
#define NON_SHAREABLE		0x0
#define OUTER_SHAREABLE		0x2
#define INNER_SHAREABLE		0x3

#define VA_TO_PA(va)		(((va) - RMM_VIRT) + RMM_PHYS)
#define PA_TO_VA(pa)		(((pa) - RMM_PHYS) + RMM_VIRT)

#endif /* __PAGETABLE_H__ */
