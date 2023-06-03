#ifndef __TABLE_H_
#define __TABLE_H_

#include <memory.h>

/*
 * The state of IPAs within the PAR that describes the following properties:
 *  - Is there a granule assigned to this IPA?
 *  - Is it mapped?
 *  - Can the host assign a new granule to this IPA?
 *
 * The values are stored in PTEs of Realms' stage 2 page tables:
 *  - When stored at level 3, the value represent the state of
 *    an individual 4KB page.
 *  - When stored level 0, 1 or 2, the value represent the state of
 *    a block of consecutive pages.
 *
 * Name:		Stored at:	Page Count in	Block's
 *					the Block:	alignement:
 * --------------------------------------------------------
 * IPA State		L3 PTE		1		4KB
 * Block 2 IPA State	L2 PTE		2**9		2MB
 * Block 1 IPA State	L1 PTE		2**18		1GB
 * Block 0 IPA State	L0 PTE		2**27		512GB
 *
 *
 * The RMM uses S/W reserved bits 56-58 in Stage 2 PTEs to store the state.
 * Note that, if some of S/W bits 56-58 are required for other features, a more
 * compact encoding for IPA states may be possible.
 */
enum ipa_state {
	/*
	 * The initial state of the address upon the Realm's creation.
	 * No DRAM granule is assigned to the address.
	 * The host may assign a new DATA granule to this address.
	 *
	 * The state can be stored in:
	 * - Invalid PTE at L3 PT, as Vacant.
	 * - Invalid PTE at L2 PT, as Block 2 Vacant.
	 * - Invalid PTE at L1 PT, as Block 1 Vacant.
	 * - Invalid PTE at L0 PT, as Block 0 Vacant.
	 *
	 * The value must be zero. (as a newly created table is zero-filled).
	 */
	IPA_STATE_VACANT = 0,
	/*
	 * A DATA granule is assigned to the address. The granule is not mapped.
	 *
	 * The state can be stored in:
	 * - Invalid PTE at L3 PT, as Absent.
	 * - Invalid PTE at L2 PT, as Block 2 Absent.
	 *
	 * The PA of the assigned granule is stored in the PTE's bits [47:12]
	 * (bits[47:21] for the case Block 2 Absent state).
	 */
	IPA_STATE_ABSENT = 1,
	/*
	 * A DATA granule is assigned and mapped at this address.
	 *
	 * The state can be stored in:
	 * - Page PTE at L3 PT, as Present.
	 * - Block PTE at L2 PT, as Block 2 Present.
	 *
	 * The PA of the assigned granule is stored in the PTE's bits [47:12]
	 * (bits[47:21] for the case Block 2 Present state).
	 */
	IPA_STATE_PRESENT = 2,
	/*
	 * The previous content of the page has been destroyed and no granule
	 * is assigned to this IPA.
	 * The host cannot assign a new DATA granule to this address. (To assign
	 * a new DATA granule to this address, the address must first transition
	 * to IPA_STATE_VACANT, see RMI.Data.Dispose).
	 *
	 * The state can be stored in:
	 * - Invalid PTE at L3 PT, as Destroyed.
	 * - Invalid PTE at L2 PT, as Block 2 Destroyed.
	 * - Invalid PTE at L1 PT, as Block 1 Destroyed.
	 * - Invalid PTE at L0 PT, as Block 0 Destroyed.
	 */
	IPA_STATE_DESTROYED = 3,
};

#define PTE_IPA_STATE_SHIFT 56
#define PTE_IPA_STATE_MASK ((unsigned long)(0x7) << PTE_IPA_STATE_SHIFT)
#define PTE_TO_IPA_STATE(pte) ((enum ipa_state) \
		(((pte) & PTE_IPA_STATE_MASK) >> PTE_IPA_STATE_SHIFT))
#define IPA_STATE_TO_PTE(ipa_state) \
		((unsigned long)(ipa_state) << PTE_IPA_STATE_SHIFT)
#define PTE_TO_PA(pte) ((unsigned long) \
		((pte) & (unsigned long)(0xfffffffff000)))

#define NR_TABLE_LEVELS	4
#define RTT_PAGE_LEVEL	3

struct tbl_walk {
	struct granule *g_llt;
	unsigned long index;
};

void table_walk_lock_unlock(struct granule *g_root,
					      unsigned long map_addr,
					      unsigned long level);

/*
 * The MMU is a separate observer, and requires that page table updates are
 * made with single-copy-atomic stores, necessitating inline assembly. For
 * consistency we use accessors for both reads and writes of page table
 * entries.
 */
static inline void pgte_write(uint64_t *pgtep, uint64_t pgte)
{
	SCA_WRITE64(pgtep, pgte);
}

static inline uint64_t pgte_read(uint64_t *pgtep)
{
	return SCA_READ64(pgtep);
}

#endif /* __TABLE_H_ */
