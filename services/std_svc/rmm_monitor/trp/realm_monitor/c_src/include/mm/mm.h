#pragma once

#define ROUNDUP(n,sz)   (((((n)-1)/(sz))+1)*(sz))
#define ROUNDDOWN(n,sz) ( (n) / (sz) * (sz) )
#define RMM_MAP_MEMORY_RANGE 0x100000000

vaddr_t phys_to_virt(paddr_t phys);
paddr_t virt_to_phys(vaddr_t virt);
void mm_primary_init(void);
void mm_secondary_init(void);
