#include <barriers.h>
#include <smc-rmi.h>

unsigned long handle_ns_smc(unsigned long function_id,
			    unsigned long arg0,
			    unsigned long arg1,
			    unsigned long arg2,
			    unsigned long arg3,
			    unsigned long arg4,
			    unsigned long arg5)
{
    // function_id = function_id & SMC_ARM_ARCH_CALL_MASK;

	// if (function_id == SMC_RMM_VERSION)
	// 	return (unsigned long)RMM_ABI_VERSION;
	// else if (function_id == SMC_RMM_GRANULE_DELEGATE)
	// 	return smc_granule_delegate(arg0);
	// else if (function_id == SMC_RMM_GRANULE_UNDELEGATE)
	// 	return smc_granule_undelegate(arg0);
	// else if (function_id == SMC_RMM_REALM_CREATE)
	// 	return smc_realm_create(arg0, arg1);
	// else if (function_id == SMC_RMM_REALM_DESTROY)
	// 	return smc_realm_destroy(arg0);
	// else if (function_id == SMC_RMM_REALM_ACTIVATE)
	// 	return smc_realm_activate(arg0);
	// else if (function_id == SMC_RMM_REC_CREATE)
	// 	return smc_rec_create(arg0, arg1, arg2, arg3);
	// else if (function_id == SMC_RMM_REC_DESTROY)
	// 	return smc_rec_destroy(arg0);
	// else if (function_id == SMC_RMM_DATA_CREATE)
	// 	return smc_data_create(arg0, arg1, arg2, arg3);
	// else if (function_id == SMC_RMM_DATA_DESTROY)
	// 	return smc_data_destroy(arg0, arg1);
	// else if (function_id == SMC_RMM_TABLE_CREATE)
	// 	return smc_table_create(arg0, arg1, arg2, arg3);
	// else if (function_id == SMC_RMM_TABLE_DESTROY)
	// 	return smc_table_destroy(arg0, arg1, arg2, arg3);
	// else if (function_id == SMC_RMM_REC_RUN)
	// 	return smc_rec_run(arg0, arg1);
	// else if (function_id == SMC_RMM_DATA_MAP)
	// 	return smc_data_map(arg0, arg1);
	// else if (function_id == SMC_RMM_DATA_UNMAP)
	// 	return smc_data_unmap(arg0, arg1);
	// else if (function_id == SMC_RMM_INTERRUPT_SIGNAL)
	// 	return smc_interrupt_signal(arg0, arg1);
	// else if (function_id == SMC_RMM_DATA_CREATE_UNKNOWN)
	// 	return smc_data_create_unknown(arg0, arg1, arg2);
	// else if (function_id == SMC_RMM_DATA_DISPOSE)
	// 	return smc_data_dispose(arg0, arg1);
	// else {
	// 	assert_cond(0UL);
	// 	return 0UL;
	// }
    return 0;
}

void handle_rmm_trap(unsigned long *regs)
{
	while (1) {
		wfe();
	}
}