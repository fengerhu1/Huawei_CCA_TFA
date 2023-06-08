#ifndef __MMU_H_
#define __MMU_H_

#include <assembler.h>

.macro	virt_offset reg vaddr
	ldr \reg, =(\vaddr - RMM_VIRT)
.endm

/*  The physical address of the mapping is dervied from VA by calculating
 *  the offset from start and adding the load address.
 */
.macro	va_to_pa reg addr
	ldr \reg, =(\addr - RMM_VIRT + RMM_PHYS)
.endm

.macro	table_1G addr, table_attrs
	.quad (\table_attrs + (\addr - RMM_VIRT + RMM_PHYS) + 0x3)
.endm

.macro	table_2M addr, table_attrs
	.quad (\table_attrs + (\addr - RMM_VIRT + RMM_PHYS) + 0x3)
.endm

.macro inv_entry count
	.fill \count, 8, 0
.endm

.macro fill_entry start
	.fill 512*8 - (. - \start) , 1, 0
.endm

/* A helper macro to add mapping of a section. The parameters are:
 *  base     - translation table base of level 2 or level 3.
 *  va_start - start of the virtual address to be mapped.
 *  va_end   - end of the virtual address to be mapped.
 *  attrs    - page table entry attributes.
 *  level    - page table entry level. For level-2, block mappings and level-3,
 *             page size mappings will be created.
 *
 *  The physical address of the mapping is dervied from VA using
 *  va_to_pa macro.
 *  The va_start and va_end size must be aligned according to the level of
 *  mapping requested.
 */
.macro map_section base, va_start, va_end, attrs, level
	va_to_pa	x0, \base
	virt_offset	x1, \va_start
	va_to_pa	x2, \va_start
	ldr		x3, =\va_end
	ldr		x4, =\va_start
	sub		x3, x3, x4
	ldr		x4, =\attrs
	bl		create_pte_l\level
	cbnz		x0, pt_error
.endm

/* A helper macro to add mapping of an IO section. This is different from the
 * section mapping macro in 2 ways:
 *  1) The PA of the mapping created is specified in the argument.
 *  2) Instead of va_end parameter, the size of the mapping is provided.
 *
 *  The parameters are:
 *  base - translation table base of level 2 or level 3
 *  va_start - start of the virtual address to be mapped.
 *  pa_start - start of the physical address to be mapped.
 *  size     - size of the mapping.
 *  attrs    - page table entry attributes
 *  level    - page table entry level. For level-2, block mappings and level-3,
 *             page size mappings will be created.
 *
 *  The va_start, pa_start and size must be aligned according to the level of
 *  mapping requested.
 */

.macro map_io base, va_start, pa_start, size, attrs, level
	va_to_pa	x0, \base
	virt_offset	x1, \va_start
	ldr		x2, =\pa_start
	ldr		x3, =\size
	ldr		x4, =\attrs
	bl		create_pte_l\level
	cbnz		x0, pt_error
.endm

#endif /* __MMU_H__ */
