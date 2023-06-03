#include <assembler.h>
#include <sizes.h>
#include <pagetable.h>

/*
 * Define a function to create page or block mappings for a VA region.
 * Takes several input values in registers:
 *  x0: base address of page table
 *  x1: virtual address range to map (must be aligned to blk_sz)
 *  x2: physical address to map to (must be aligned to blk_sz)
 *  x3: size of the region to map (must be aligned to blk_sz)
 *  x4: page entry attributes
 *
 * Clobbers: x0-x10
 * Return value in x0 as follows:
 *    > 0: The remaining page size the routine was unable to create mappings for.
 *    = 0: Success.
 *    < 0 for error.
 */
.macro	create_pte_level name, mask, shift, blk_sz, valid
ENTRY(\name):
	and     x8, x1, #\mask
	lsr	x8, x8, #(\shift - 3)	/* x8 = L3 idx in byte offset */

1: /* loop for next entry */
	cbz     x3, 3f			/* if size > 0 */
	cmp     x8, #4096		/* if pte < 256 */
	bge     3f			/* stop if we overflow current descs */
	add	x9, x0, x8		/* x9 = entryp */
	ldr     x10, [x9]		/* x10 = entry */
	cbnz    x10, 2f			/* error overlap */

	/* allocate block */
	orr     x10, x2, x4
	orr     x10, x10, #\valid
	str     x10, [x9]
	add     x2, x2, #\blk_sz
	add     x8, x8, #8
	sub     x3, x3, #\blk_sz
	b       1b
2:
	mov	x0, -1
        ret
3:
	mov	x0, x3
        ret
ENDPROC(\name)
.endm

/* create function for page/block entries for level-2 and level3 */
create_pte_level create_pte_l3, L3_MASK, L3_SHIFT, L3_BLK_SZ, L3_PAGE_ENTRY

create_pte_level create_pte_l2, L2_MASK, L2_SHIFT, L2_BLK_SZ, L2_BLOCK_ENTRY
