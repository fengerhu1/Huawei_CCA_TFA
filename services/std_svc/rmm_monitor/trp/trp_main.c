/*
 * Copyright (c) 2021-2022, Arm Limited and Contributors. All rights reserved.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */


#include <common/debug.h>
#include <plat/common/platform.h>
#include <services/rmm_core_manifest.h>
#include <services/rmmd_svc.h>
#include <services/trp/platform_trp.h>
#ifdef PLAT_QEMU
#include <services/trp/trp_helpers.h>
#endif

#ifdef PLAT_FVP
#include <trp_helpers.h>
#endif
#include "trp_private.h"

#include <platform_def.h>

/* Parameters received from the previous image */
static unsigned int trp_boot_abi_version;
static uintptr_t trp_shared_region_start;

/* Parameters received from boot manifest */
uint32_t trp_boot_manifest_version;

/*******************************************************************************
 * Setup function for TRP.
 ******************************************************************************/
void trp_setup(uint64_t x0,
	       uint64_t x1,
	       uint64_t x2,
	       uint64_t x3)
{
	/*
	 * Validate boot parameters
	 *
	 * According to the Boot Interface ABI v.0.1,
	 * the parameters received from EL3 are:
	 * x0: CPUID (verified earlier, so not used)
	 * x1: Boot Interface version
	 * x2: PLATFORM_CORE_COUNT
	 * x3: Pointer to the shared memory area.
	 */

	(void)x0;

	if (TRP_RMM_EL3_VERSION_GET_MAJOR(x1) != TRP_RMM_EL3_ABI_VERS_MAJOR) {
		trp_boot_abort(E_RMM_BOOT_VERSION_MISMATCH);
	}

	if ((void *)x3 == NULL) {
		trp_boot_abort(E_RMM_BOOT_INVALID_SHARED_BUFFER);
	}

	if (x2 > TRP_PLATFORM_CORE_COUNT) {
		trp_boot_abort(E_RMM_BOOT_CPUS_OUT_OF_RANGE);
	}

	trp_boot_abi_version = x1;
	trp_shared_region_start = x3;
	flush_dcache_range((uintptr_t)&trp_boot_abi_version,
			   sizeof(trp_boot_abi_version));
	flush_dcache_range((uintptr_t)&trp_shared_region_start,
			   sizeof(trp_shared_region_start));

	/* Perform early platform-specific setup */
	#ifdef PLAT_QEMU
	trp_early_platform_setup_qemu();
	#endif

	#ifdef PLAT_FVP
	trp_early_platform_setup((struct rmm_manifest *)trp_shared_region_start);
	#endif
}

#ifdef PLAT_QEMU
int trp_validate_warmboot_args(uint64_t x0, uint64_t x1,
			       uint64_t x2, uint64_t x3)
{
	/*
	 * Validate boot parameters for warm boot
	 *
	 * According to the Boot Interface ABI v.0.1, the parameters
	 * received from EL3 during warm boot are:
	 *
	 * x0: CPUID (verified earlier so not used here)
	 * [x1:x3]: RES0
	 */

	(void)x0;

	return ((x1 | x2 | x3) == 0UL) ? 0 : E_RMM_BOOT_UNKNOWN;
}
#endif

extern void rust_printf();
extern void rust_test_alloc();
extern void allocator_init();
extern void mm_primary_init();
extern void virt_primary_init();
extern void init_console();
extern void init_granule();
extern unsigned long smc_realm_create(unsigned long rd_addr, unsigned long rlm_para_addr);
extern unsigned long smc_realm_destroy(unsigned long rd_addr);
extern unsigned long smc_realm_activate(unsigned long rd_addr);
extern unsigned long smc_table_create(unsigned long tbl_addr,
			     unsigned long rd_addr,
			     unsigned long map_addr,
			     unsigned long level);
extern unsigned long smc_granule_delegate(unsigned long addr);
extern unsigned long smc_granule_undelegate(unsigned long addr);
extern unsigned long smc_table_destroy(unsigned long rtt_addr,
				unsigned long rd_addr,
				unsigned long map_addr,
				unsigned long level);
extern unsigned long smc_data_map(unsigned long rd_addr,
			   unsigned long map_addr);
extern unsigned long smc_data_unmap(unsigned long rd_addr,
			     unsigned long map_addr);
extern unsigned long smc_data_create(unsigned long data_addr,
			      unsigned long rd_addr,
			      unsigned long src_addr,
			      unsigned long map_addr);
extern unsigned long smc_data_destroy(unsigned long map_addr,
			       unsigned long rd_addr);
extern unsigned long smc_data_create_unknown(unsigned long data_addr,
				      unsigned long rd_addr,
				      unsigned long map_addr);
extern unsigned long smc_data_create_shared(unsigned long data_addr,
				      unsigned long rd_addr,
				      unsigned long map_addr);
extern unsigned long smc_data_dispose(unsigned long rd_addr, unsigned long rec_addr);
extern unsigned long smc_rec_create(unsigned long rec_addr,
			     unsigned long rd_addr,
			     unsigned long mpidr,
			     unsigned long rec_params_addr);
extern unsigned long smc_rec_destroy(unsigned long rec_addr);
extern unsigned long smc_rec_run(unsigned long rec_addr,
			  unsigned long rec_run_addr);

void print_info(const char *fmt, ...) {
	va_list args;
	va_start(args, fmt);
	vprintf(fmt, args);
	va_end(args);
}

void print_string(char* string) {
	print_info("%s", string);
}

void test_secondary() {
	INFO("secondary boot here \n");
}

#ifdef PLAT_QEMU
/*******************************************************************************
 * Setup function for TRP.
 ******************************************************************************/
void trp_setup_qemu(void)
{
	/* Perform early platform-specific setup */
	trp_early_platform_setup_qemu();
	NOTICE("TRP: trp_early_platform_setup_qemu\n");
	trp_plat_arch_setup();
	NOTICE("TRP: trp_plat_arch_setup\n");
}
#endif

/* Main function for TRP */
void trp_main(void)
{
	NOTICE("TRP: %s\n", version_string);
	NOTICE("TRP: %s\n", build_message);
	NOTICE("TRP: Supported RMM-EL3 Interface ABI: v.%u.%u\n",
		TRP_RMM_EL3_ABI_VERS_MAJOR, TRP_RMM_EL3_ABI_VERS_MINOR);
	NOTICE("TRP: Boot Manifest Version: v.%u.%u\n",
		RMMD_GET_MANIFEST_VERSION_MAJOR(trp_boot_manifest_version),
		RMMD_GET_MANIFEST_VERSION_MINOR(trp_boot_manifest_version));
	INFO("TRP: Memory base: 0x%lx\n", (unsigned long)RMM_BASE);
	INFO("TRP: Shared region base address: 0x%lx\n",
			(unsigned long)trp_shared_region_start);
	INFO("TRP: Total size: 0x%lx bytes\n",
			(unsigned long)(RMM_END - RMM_BASE));
	INFO("TRP: RMM-EL3 Interface ABI reported by EL3: v.%u.%u\n",
		TRP_RMM_EL3_VERSION_GET_MAJOR(trp_boot_abi_version),
		TRP_RMM_EL3_VERSION_GET_MINOR(trp_boot_abi_version));
	print_info("hello world 1\n");
	mm_primary_init();
	print_info("hello world 2\n");
	allocator_init();
	print_info("hello world 3\n");
	init_console();
	print_info("hello world 4\n");
	init_granule();
	print_info("hello world 5\n");
	virt_primary_init();
	print_info("hello world 6\n");
	rust_printf();
	print_info("hello world 7\n");
	rust_test_alloc();
	print_info("hello world 8\n");
}

#ifdef PLAT_QEMU
void trp_enable_mmu(void)
{
	int linear_id = plat_my_core_pos();
	trp_plat_arch_enable_mmu(linear_id);
}
#endif

/*******************************************************************************
 * Returning RMI version back to Normal World
 ******************************************************************************/
static void trp_ret_rmi_version(struct trp_smc_result *smc_ret)
{
	VERBOSE("RMM version is %u.%u\n", RMI_ABI_VERSION_MAJOR,
					  RMI_ABI_VERSION_MINOR);
	smc_ret->x[0] = RMI_ABI_VERSION;
}

/*******************************************************************************
 * Transitioning granule of NON-SECURE type to REALM type
 ******************************************************************************/
static void trp_asc_mark_realm(unsigned long long x1,
				struct trp_smc_result *smc_ret)
{
	VERBOSE("Delegating granule 0x%llx\n", x1);
	smc_ret->x[0] = trp_smc(set_smc_args(RMM_GTSI_DELEGATE, x1,
						0UL, 0UL, 0UL, 0UL, 0UL, 0UL));	
	smc_granule_delegate(x1);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Granule transition from NON-SECURE type to REALM type "
			"failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Transitioning granule of REALM type to NON-SECURE type
 ******************************************************************************/
static void trp_asc_mark_nonsecure(unsigned long long x1,
				   struct trp_smc_result *smc_ret)
{
	VERBOSE("Undelegating granule 0x%llx\n", x1);
	smc_granule_undelegate(x1);
	smc_ret->x[0] = trp_smc(set_smc_args(RMM_GTSI_UNDELEGATE, x1, 0UL, 0UL, 0UL, 0UL, 0UL, 0UL));

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Granule transition from REALM type to NON-SECURE type "
			"failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * create realm metadata with the given parameters
 ******************************************************************************/
static void trp_realm_create(unsigned long long x1, unsigned long long x2, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_realm_create(x1, x2);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Create Realm is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * destroy realm metadata with the given parameters
 ******************************************************************************/
static void trp_realm_destroy(unsigned long long x1, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_realm_destroy(x1);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Destory Realm is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * activate realm metadata with the given parameters
 ******************************************************************************/
static void trp_realm_activate(unsigned long long x1, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_realm_activate(x1);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Activate Realm is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * create realm page table
 ******************************************************************************/
static void trp_table_create(unsigned long long x1, unsigned long long x2, unsigned long long x3, unsigned long long x4, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_table_create(x1, x2, x3, x4);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Create realm page table is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * destroy realm page table
 ******************************************************************************/
static void trp_table_destroy(unsigned long long x1, unsigned long long x2, unsigned long long x3, unsigned long long x4, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_table_destroy(x1, x2, x3, x4);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Destroy realm page table is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Validate the data page in the RTT
 ******************************************************************************/
static void trp_data_map(unsigned long long x1, unsigned long long x2, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_data_map(x1, x2);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Map the data page is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Invalidate the data page in the RTT
 ******************************************************************************/
static void trp_data_unmap(unsigned long long x1, unsigned long long x2, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_data_unmap(x1, x2);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Unmap the data page is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Create a data page and map to the rtt, before data_map
 ******************************************************************************/
static void trp_data_create(unsigned long long x1, unsigned long long x2, unsigned long long x3, unsigned long long x4, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_data_create(x1, x2, x3, x4);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Create data page is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Destroy a data page and clear the pte entry, after data_unmap
 ******************************************************************************/
static void trp_data_destroy(unsigned long long x1, unsigned long long x2, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_data_destroy(x1, x2);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Destroy data page is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Create an unknown data page and map to the rtt, before data_map
 ******************************************************************************/
static void trp_data_create_unknown(unsigned long long x1, unsigned long long x2, unsigned long long x3, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_data_create_unknown(x1, x2, x3);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Create unknown data page is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Dispose an unknown data page and clear the pte entry, after data_unmap
 ******************************************************************************/
static void trp_data_dispose(unsigned long long x1, unsigned long long x2,
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_data_dispose(x1, x2);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Dispose unknown data page is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Create an unknown data page and map to the rtt, before data_map
 ******************************************************************************/
static void trp_data_create_shared(unsigned long long x1, unsigned long long x2, unsigned long long x3, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_data_create_shared(x1, x2, x3);
	if (smc_ret->x[0] != 0ULL) {
		ERROR("Create shared data page is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Create realm context with initial register value
 ******************************************************************************/
static void trp_rec_create(unsigned long long x1, unsigned long long x2, unsigned long long x3, unsigned long long x4, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_rec_create(x1, x2, x3, x4);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Create rec is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Destroy realm context
 ******************************************************************************/
static void trp_rec_destroy(unsigned long long x1, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_rec_destroy(x1);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Destroy rec is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Run realm
 ******************************************************************************/
static void trp_rec_run(unsigned long long x1, unsigned long long x2, 
					struct trp_smc_result *smc_ret)
{
	smc_ret->x[0] = smc_rec_run(x1, x2);

	if (smc_ret->x[0] != 0ULL) {
		ERROR("Run rec is failed 0x%llx\n", smc_ret->x[0]);
	}
}

/*******************************************************************************
 * Main RMI SMC handler function
 ******************************************************************************/
void trp_rmi_handler(unsigned long fid,
		     unsigned long long x1, unsigned long long x2,
		     unsigned long long x3, unsigned long long x4,
		     unsigned long long x5, unsigned long long x6,
		     struct trp_smc_result *smc_ret)
{
	/* Not used in the current implementation */
	(void)x2;
	(void)x3;
	(void)x4;
	(void)x5;
	(void)x6;

	switch (fid) {
	case RMI_RMM_REQ_VERSION:
		trp_ret_rmi_version(smc_ret);
		break;
	case RMI_RMM_GRANULE_DELEGATE:
		trp_asc_mark_realm(x1, smc_ret);
		break;
	case RMI_RMM_GRANULE_UNDELEGATE:
		trp_asc_mark_nonsecure(x1, smc_ret);
		break;
	case RMI_RMM_REALM_CREATE:
		trp_realm_create(x1, x2, smc_ret);
		break;
	case RMI_RMM_REALM_DESTROY:
		trp_realm_destroy(x1, smc_ret);
		break;
	case RMI_RMM_REALM_ACTIVATE:
		trp_realm_activate(x1, smc_ret);
		break;
	case RMI_RMM_TABLE_CREATE:
		trp_table_create(x1,x2,x3,x4, smc_ret);
		break;
	case RMI_RMM_TABLE_DESTROY:
		trp_table_destroy(x1, x2, x3, x4, smc_ret);
		break;
	case RMI_RMM_DATA_MAP:
		trp_data_map(x1, x2, smc_ret);
		break;
	case RMI_RMM_DATA_UNMAP:
		trp_data_unmap(x1, x2, smc_ret);
		break;
	case RMI_RMM_DATA_CREATE:
		trp_data_create(x1, x2, x3, x4, smc_ret);
		break;
	case RMI_RMM_DATA_DESTROY:
		trp_data_destroy(x1, x2, smc_ret);
		break;
	case RMI_RMM_REC_CREATE:
		trp_rec_create(x1, x2, x3, x4, smc_ret);
		break;
	case RMI_RMM_REC_DESTROY:
		trp_rec_destroy(x1, smc_ret);
		break;
	case RMI_RMM_REC_RUN:
		trp_rec_run(x1, x2, smc_ret);
		break;
	case RMI_RMM_DATA_CREATE_UNKNOWN:
		trp_data_create_unknown(x1, x2, x3, smc_ret);
		break;
	case RMI_RMM_DATA_DISPOSE:
		trp_data_dispose(x1, x2, smc_ret);
		break;
	case RMI_RMM_MAP_NS:
		trp_data_create_shared(x1, x2, x3, smc_ret);
		break;
	default:
		ERROR("Invalid SMC code to %s, FID %lx\n", __func__, fid);
		smc_ret->x[0] = SMC_UNK;
	}
}

void print_sp(unsigned long sp) {
	INFO("Debug: SP stack is %lx\n", sp);	

}

void handle_rmm_trap(unsigned long *regs, unsigned long esr, unsigned long address)
{
	ERROR("handle_rmm_trap\n");
	ERROR("ESR:%lx address:%lx \n", esr, address);
	if (esr == 0x96000047) {
		ERROR("ns bit is error\n");
	}
	while (1) {
		
	}
}
