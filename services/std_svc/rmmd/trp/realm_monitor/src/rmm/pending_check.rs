use crate::rmm::rvic_util::{
    RecRvicState,
	INTID_VTIMER_EL1,
	INTID_PTIMER_EL1,
};

use crate::io::Write;

use crate::rmm::rsi_util::{
	handle_realm_rsi,
};

use crate::rmm::granule_util::{
    Granule,
    ErrorStatus,
    BufferSlot,
    NR_GRANULES,
    GRANULE_SHIFT,
    GRANULE_SIZE,
    GranuleUtil,
    GranuleState,
	MEM0_PHYS,
};

use crate::rmm::smc_rmi::{
	RecParams,
	RecRun,
	REC_RUN_HVC_NR_GPRS,
};

use crate::rmm::rmm_smc::{
	pack_return_code,
};

use crate::rmm::realm_util::{
    Rd,
};

use crate::rmm::context_switch::{
	save_ns_state_sysreg_state,
	restore_ns_state_sysreg_state,
	restore_sysreg_state,
	save_sysreg_state,
};

use crate::rmm::sysreg::{
	MPIDR_EL1_AFF0,
	MPIDR_EL1_AFF1,
	MPIDR_EL1_AFF2,
	MPIDR_EL1_AFF3,
	MPIDR_EL1_AFF0_SHIFT,
	MPIDR_EL1_AFF1_SHIFT,
	MPIDR_EL1_AFF2_SHIFT,
	MPIDR_EL1_AFF3_SHIFT,
	PSTATE_INIT,
	PMCR_EL0_RES1,
	SCTLR_EL1_FLAGS,
	MDSCR_EL1_TDCC_BIT,
	CNTHCTL_EL2_NO_TRAPS,
	TTBRx_EL2_BADDR_SHIFT,
	TTBRx_EL2_BADDR_WIDTH,
	HCR_FLAGS,
	VTCR_FLAGS,
	ESR_EL2_EC_MASK,
	ESR_EL2_EC_HVC,
	NOT_ICC_SRE_EL2_ENABLE,
	ICC_HPPIR1_EL1_INTID,
	ESR_EL2_EC_WFX,
	ESR_EL2_WFx_TI_BIT,
	ESR_EL2_xVC_IMM_MASK,
	ESR_EL2_EC_SMC,
	ESR_EL2_EC_SYSREG,
	ESR_EL2_EC_INST_ABORT,
	ESR_EL2_EC_DATA_ABORT,
	ESR_EL2_ABORT_SRT_MASK,
	ESR_EL2_ABORT_SRT_SHIFT,
	ESR_EL2_ABORT_ISV_BIT,
	ESR_EL2_ABORT_SAS_MASK,
	ESR_EL2_ABORT_SAS_SHIFT,
	ESR_EL2_ABORT_SAS_BYTE_VAL,
	ESR_EL2_ABORT_SAS_HWORD_VAL,
	ESR_EL2_ABORT_SAS_WORD_VAL,
	ESR_EL2_ABORT_SAS_DWORD_VAL,
	ESR_EL2_ABORT_SSE_BIT,
	ESR_EL2_ABORT_SF_BIT,
	ESR_EL2_ABORT_WNR_BIT,
};

use crate::rmm::rmm_util::{
    granule_map_with_id,
    granule_map_with_id_state,
    buffer_unmap,
    granule_map_zero,
    RmmUtil,
};

use crate::rmm::abs_util::{
    VPERCPU_LOCK,
};

use crate::rmm::smc_rmi::{
	REC_CREATE_NR_GPRS,
};

