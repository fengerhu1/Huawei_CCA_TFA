#ifndef __ABS_H_
#define __ABS_H_
#include <const.h>
#include <platform.h>
#include <buffer.h>
#include <sysreg.h>
#include <mm/mm.h>

#define PGTE_PAGE	(3UL << 0)
#define PGTE_NS		(1UL << 5)
#define PGTE_AP_RW_NA	(0UL << 6)
#define PGTE_SH_IS	(3UL << 8)
#define PGTE_AF		(1UL << 10)
#define PGTE_NG		(1UL << 11)
#define PGTE_PXN	(1UL << 53)
#define PGTE_XN		(1UL << 54)

// #define PGTE_SLOT \
// 	(PGTE_PAGE | PGTE_AP_RW_NA | PGTE_SH_IS | PGTE_AF | PGTE_NG | \
// 	 PGTE_PXN | PGTE_XN)

//FIXME: Do we need nG bit?
#define PGTE_SLOT \
	(PGTE_PAGE | PGTE_AP_RW_NA | PGTE_SH_IS | PGTE_AF | (4UL<<2) )

#define PGTABLE_SIZE		GRANULE_SIZE
#define NR_PGTABLE_ENTRIES	(PGTABLE_SIZE / sizeof(uint64_t))

// #include <bits/types.h>

// typedef __uint8_t uint8_t;
// typedef __uint16_t uint16_t;
// typedef __uint32_t uint32_t;
// typedef __uint64_t uint64_t;

struct pgtable {
	uint64_t pgte[NR_PGTABLE_ENTRIES];
} __attribute__((aligned(PGTABLE_SIZE)));

uint64_t *slot_to_pgte(enum BufferSlot slot);

uint64_t *va_to_pgte(void *va);

static inline void *slot_to_va(enum BufferSlot slot)
{
	unsigned long idx, offset;

	assert(slot < NR_CPU_SLOTS);
	idx = cpuid() * NR_CPU_SLOTS + slot;
	offset = idx * GRANULE_SIZE;
	return (void *)RMM_MAP_MEMORY_RANGE + offset;
}

#define VA_TO_PA(v)	(((unsigned long)(v)) - RMM_VIRT + RMM_PHYS)
#define PA_TO_VA(p)	(((unsigned long)(p)) - RMM_PHYS + RMM_VIRT)

#define TABLE(t)	(VA_TO_PA(&(t)) + 3)

/*
 * The number of GPRs (starting from X0) that are
 * configured by the host when a REC is created.
 */
#define REC_CREATE_NR_GPRS		8

struct rec_params {
	unsigned long gprs[REC_CREATE_NR_GPRS];
	unsigned long pc;
	unsigned long flags;
};

#define REC_RUN_HVC_NR_GPRS 7

struct rec_run {
	/* output values*/
	unsigned long exit_reason;
	unsigned long esr;
	unsigned long far;
	unsigned long hpfar;
	unsigned long emulated_write_val;
	unsigned long gprs[REC_RUN_HVC_NR_GPRS];
	unsigned long disposed_addr;
	/* input values */
	unsigned long is_emulated_mmio;
	unsigned long emulated_read_val;
	unsigned long target_rec;
};

#endif