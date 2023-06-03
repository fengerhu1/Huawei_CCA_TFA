/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */
#ifndef __SMC_RMI_H_
#define __SMC_RMI_H_

/*
 * This file describes the Realm Services Interface (RSI) Application Binary
 * Interface (ABI) for SMC calls made from within the Realm to the RMM and
 * serviced by the RMM.
 *
 * See doc/rmm_interface.md for more details.
 */

/* The version number of the RMI implementation.  Increase this whenever the
 * binary format or semantics of the SMC calls change */
#define RMM_ABI_VERSION		19

// We allocate all RMM calls as function IDs within the
// SMC64: ARM Architecture Calls
// category defined in the SMCCC.

#define SMC_ARM_ARCH_CALL_BASE	0xC0000000
#define SMC_ARM_ARCH_CALL_MASK	0xFFFF

#define RMI_SUCCESS			0
#define RMI_ERROR_INPUT			1
#define RMI_ERROR_MEMORY		2
#define RMI_ERROR_ALIAS			3
#define RMI_ERROR_IN_USE		4
#define RMI_ERROR_REALM_STATE		5
#define RMI_ERROR_OWNER			6
#define RMI_ERROR_REC			7
#define RMI_ERROR_RTT_WALK		8
#define RMI_ERROR_RTT_ENTRY		9

#define SMC_RMM_VERSION			0
#define SMC_RMM_GRANULE_DELEGATE	1
#define SMC_RMM_GRANULE_UNDELEGATE	2

struct realm_params {
	unsigned long par_base;
	unsigned long par_size;
	unsigned long rec_list_addr; /* rec list granule address */
	unsigned long table_addr; /* Translation Table Base Granule address */
	unsigned long measurement_algo; /* Measurement algorithm */
};

#define REALM_MEASUREMENT_ALGO_ZERO	0
#define REALM_MEASUREMENT_ALGO_SHA256	1

/*
 * arg0 == RD address
 * arg1 == struct realm_params addr
 */
#define SMC_RMM_REALM_CREATE		3

/*
 * arg0 == RD address
 */
#define SMC_RMM_REALM_DESTROY		4

/*
 * arg0 == RD address
 */
#define SMC_RMM_REALM_ACTIVATE		5

/*
 * The number of GPRs (starting from X0) that are
 * configured by the host when a REC is created.
 */
#define REC_CREATE_NR_GPRS		8

#define REC_PARAMS_FLAG_RUNNABLE	(1UL << 0)

struct rec_params {
	unsigned long gprs[REC_CREATE_NR_GPRS];
	unsigned long pc;
	unsigned long flags;
};
/*
 * arg0 == REC address
 * arg1 == RD address
 * arg2 == MPIDR_EL1 value
 * arg3 == struct rmm_rec address
 */
#define SMC_RMM_REC_CREATE		6

/*
 * arg0 == REC address
 */
#define SMC_RMM_REC_DESTROY		7

/*
 * arg0 == data address
 * arg1 == RD address
 * arg2 == SRC address
 * arg3 == map address
 */
#define SMC_RMM_DATA_CREATE		8

/*
 * arg0 == map address
 * arg1 == RD address
 */
#define SMC_RMM_DATA_DESTROY		9

/*
 * arg0 == table address
 * arg1 == RD address
 * arg2 == map address
 * arg3 == level
 */
#define SMC_RMM_TABLE_CREATE		10

/*
 * arg0 == table address
 * arg1 == RD address
 * arg2 == map address
 * arg3 == level
 */
#define SMC_RMM_TABLE_DESTROY		11

/*
 * The number of GPRs (starting from X0) per voluntary exit context.
 * Per SMCCC.
 */
#define REC_RUN_HVC_NR_GPRS		7

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

/*
 * arg0 == rec address
 * arg1 == rec_run address
 */
#define SMC_RMM_REC_RUN			12

#define EXIT_REASON_SYNC			0
#define EXIT_REASON_IRQ				1
#define EXIT_REASON_FIQ				2
#define EXIT_REASON_PSCI			3
#define EXIT_REASON_REC_INTERRUPT_PENDING	4
#define EXIT_REASON_RSI_DISPOSE			5

/*
 * arg0 == RD address
 * arg1 == map address
 */
#define SMC_RMM_DATA_MAP		13

/*
 * arg0 == RD address
 * arg1 == map address
 */
#define SMC_RMM_DATA_UNMAP		14


/*
 * arg0 == RD address
 * arg1 == INTID
 */
#define SMC_RMM_INTERRUPT_SIGNAL	15

/*
 * arg0 == data address
 * arg1 == RD address
 * arg2 == map address
 */
#define SMC_RMM_DATA_CREATE_UNKNOWN	16

/*
 * arg0 == RD
 * arg1 == REC
 */
#define SMC_RMM_DATA_DISPOSE		17

/*
 * arg0 == ns address
 * arg1 == RD address
 * arg2 == map address
 */
#define SMC_RMM_MAP_NS			18

/*
 * arg0 == ns address
 * arg1 == RD address
 * arg2 == map address
 */
#define SMC_RMM_UNMAP_NS		19

#define SMC_RMM_NUM_CALLS		20

#endif /* __SMC_RMI_H_ */
