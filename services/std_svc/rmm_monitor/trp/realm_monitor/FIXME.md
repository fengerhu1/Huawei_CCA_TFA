2. In allocator.rs RMM_HEAP_SIZE is larger than granule size (2 times of the total granule size) current 4M
3. In granule.rs the MEM0_SIZE is 2*512M = 1G for tf-a-test, 2G for linux;
4. In granule.rs we need to use atomic set in granule lock
5. In mmu_helper.S, we disable the SCTLR_EL2_I and SCTLR_EL2_C when enable mmu
6. In rec_util.rs, ignore the Ignore mmio emulation in the rec_run
7. In rec_util.rs, ignore data abort, inst abort and sysreg abort in handle_exception_sync
8. In rec_util.rs, ignore handle_realm_rsi in handle_exception_sync
9.  In granule.rs, MEM0_PHYS = 0x00000000c0000000 for linux; 0x000000080000000 for tf-a-test
10. In table_util.rs, set the PAR offset into hash_data_header[1] for data granule measurement



Fixed:
1. (FIXED) In mm.c: activate_mmu() is annotated
2. (FIXED) In rmm_util.rs ignore the memzero in function granule_map_zero
3.  (FIXED) In rmm_util.rs granule_map_zero is not active

