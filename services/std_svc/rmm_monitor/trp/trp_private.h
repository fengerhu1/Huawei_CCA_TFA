/*
 * Copyright (c) 2021-2022, Arm Limited and Contributors. All rights reserved.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#ifndef TRP_PRIVATE_H
#define TRP_PRIVATE_H

#include <services/rmmd_svc.h>
#include <services/trp/trp_helpers.h>

/* Definitions for RMM-EL3 Interface ABI VERSION */
#define TRP_RMM_EL3_ABI_VERS_MAJOR	RMM_EL3_IFC_VERSION_MAJOR
#define TRP_RMM_EL3_ABI_VERS_MINOR	RMM_EL3_IFC_VERSION_MINOR
#define TRP_RMM_EL3_ABI_VERS	(((TRP_RMM_EL3_ABI_VERS_MAJOR & 0x7FFF) << 16) | \
				 (TRP_RMM_EL3_ABI_VERS_MINOR & 0xFFFF))

#define TRP_PLATFORM_CORE_COUNT		PLATFORM_CORE_COUNT

#ifndef __ASSEMBLER__

#include <stdint.h>

#define write_trp_arg(args, offset, val) (((args)->regs[offset >> 3])	\
					 = val)
/* RMI SMC64 FIDs handled by the TRP */
#define RMI_RMM_REQ_VERSION		SMC64_RMI_FID(U(0))
#define RMI_RMM_GRANULE_DELEGATE	SMC64_RMI_FID(U(1))
#define RMI_RMM_GRANULE_UNDELEGATE	SMC64_RMI_FID(U(2))
#define RMI_RMM_REALM_CREATE	SMC64_RMI_FID(U(3))
#define RMI_RMM_REALM_DESTROY	SMC64_RMI_FID(U(4))
/*
 * arg0 == RD address
 */
#define RMI_RMM_REALM_ACTIVATE		SMC64_RMI_FID(U(5))

/*
 * arg0 == REC address
 * arg1 == RD address
 * arg2 == MPIDR_EL1 value
 * arg3 == struct rmm_rec address
 */
#define RMI_RMM_REC_CREATE		SMC64_RMI_FID(U(6))

/*
 * arg0 == REC address
 */
#define RMI_RMM_REC_DESTROY		SMC64_RMI_FID(U(7))

/*
 * arg0 == data address
 * arg1 == RD address
 * arg2 == SRC address
 * arg3 == map address
 */
#define RMI_RMM_DATA_CREATE		SMC64_RMI_FID(U(8))

/*
 * arg0 == map address
 * arg1 == RD address
 */
#define RMI_RMM_DATA_DESTROY		SMC64_RMI_FID(U(9))

/*
 * arg0 == table address
 * arg1 == RD address
 * arg2 == map address
 * arg3 == level
 */
#define RMI_RMM_TABLE_CREATE		SMC64_RMI_FID(U(10))

/*
 * arg0 == table address
 * arg1 == RD address
 * arg2 == map address
 * arg3 == level
 */
#define RMI_RMM_TABLE_DESTROY		SMC64_RMI_FID(U(11))

/*
 * arg0 == rec address
 * arg1 == rec_run address
 */
#define RMI_RMM_REC_RUN			SMC64_RMI_FID(U(12))

/*
 * arg0 == RD address
 * arg1 == map address
 */
#define RMI_RMM_DATA_MAP		SMC64_RMI_FID(U(13))

/*
 * arg0 == RD address
 * arg1 == map address
 */
#define RMI_RMM_DATA_UNMAP		SMC64_RMI_FID(U(14))


/*
 * arg0 == RD address
 * arg1 == INTID
 */
#define RMI_RMM_INTERRUPT_SIGNAL	SMC64_RMI_FID(U(15))

/*
 * arg0 == data address
 * arg1 == RD address
 * arg2 == map address
 */
#define RMI_RMM_DATA_CREATE_UNKNOWN	SMC64_RMI_FID(U(16))

/*
 * arg0 == RD
 * arg1 == REC
 */
#define RMI_RMM_DATA_DISPOSE		SMC64_RMI_FID(U(17))

/*
 * arg0 == ns address
 * arg1 == RD address
 * arg2 == map address
 */
#define RMI_RMM_MAP_NS			SMC64_RMI_FID(U(18))

/*
 * arg0 == ns address
 * arg1 == RD address
 * arg2 == map address
 */
#define RMI_RMM_UNMAP_NS		SMC64_RMI_FID(U(19))

#define RMI_RMM_NUM_CALLS		SMC64_RMI_FID(U(20))


/* Definitions for RMI VERSION */
#define RMI_ABI_VERSION_MAJOR		U(0x0)
#define RMI_ABI_VERSION_MINOR		U(0x0)
#define RMI_ABI_VERSION			(((RMI_ABI_VERSION_MAJOR & 0x7FFF) \
								  << 16) | \
					 (RMI_ABI_VERSION_MINOR & 0xFFFF))

#define TRP_RMM_EL3_VERSION_GET_MAJOR(x)		\
				RMM_EL3_IFC_VERSION_GET_MAJOR((x))
#define TRP_RMM_EL3_VERSION_GET_MINOR(x)		\
				RMM_EL3_IFC_VERSION_GET_MAJOR_MINOR((x))

/* Helper to issue SMC calls to BL31 */
uint64_t trp_smc(trp_args_t *);

/* The main function to executed only by Primary CPU */
void trp_main(void);

/* Setup TRP. Executed only by Primary CPU */
void trp_setup(uint64_t x0,
	       uint64_t x1,
	       uint64_t x2,
	       uint64_t x3);

/* Validate arguments for warm boot only */
int trp_validate_warmboot_args(uint64_t x0, uint64_t x1,
			       uint64_t x2, uint64_t x3);

#endif /* __ASSEMBLER__ */
#endif /* TRP_PRIVATE_H */
