#pragma once

#define ARM_SMCCC_STD_CALL	        0
#define ARM_SMCCC_FAST_CALL	        1
#define ARM_SMCCC_KVM_TRAP_CALL	    2
#define ARM_SMCCC_TYPE_SHIFT		31UL

#define ARM_SMCCC_SMC_32		0UL
#define ARM_SMCCC_SMC_64	    	1UL
#define ARM_SMCCC_CALL_CONV_SHIFT	30

#define ARM_SMCCC_OWNER_MASK		0x3F
#define ARM_SMCCC_OWNER_SHIFT		24

#define ARM_SMCCC_FUNC_MASK		    0xFFFF

#define ARM_SMCCC_IS_FAST_CALL(smc_val)	\
	((smc_val) & (ARM_SMCCC_FAST_CALL << ARM_SMCCC_TYPE_SHIFT))
#define ARM_SMCCC_IS_64(smc_val) \
	((smc_val) & (ARM_SMCCC_SMC_64 << ARM_SMCCC_CALL_CONV_SHIFT))
#define ARM_SMCCC_FUNC_NUM(smc_val)	((smc_val) & ARM_SMCCC_FUNC_MASK)
#define ARM_SMCCC_OWNER_NUM(smc_val) \
	(((smc_val) >> ARM_SMCCC_OWNER_SHIFT) & ARM_SMCCC_OWNER_MASK)

#define ARM_SMCCC_CALL_VAL(type, calling_convention, owner, func_num) \
	(((type) << ARM_SMCCC_TYPE_SHIFT) | \
	((calling_convention) << ARM_SMCCC_CALL_CONV_SHIFT) | \
	(((owner) & ARM_SMCCC_OWNER_MASK) << ARM_SMCCC_OWNER_SHIFT) | \
	((func_num) & ARM_SMCCC_FUNC_MASK))

#define ARM_SMCCC_OWNER_ARCH		0
#define ARM_SMCCC_OWNER_CPU	    	1
#define ARM_SMCCC_OWNER_SIP		    2
#define ARM_SMCCC_OWNER_OEM	    	3
#define ARM_SMCCC_OWNER_STANDARD	4
#define ARM_SMCCC_OWNER_HYPERVISOR	5
#define ARM_SMCCC_OWNER_TRUSTED_APP	48
#define ARM_SMCCC_OWNER_TRUSTED_APP_END	49
#define ARM_SMCCC_OWNER_TRUSTED_OS	50
#define ARM_SMCCC_OWNER_TRUSTED_OS_END	63

#define ARM_SMCCC_QUIRK_NONE		0
#define ARM_SMCCC_QUIRK_QCOM_A6		1 /* Save/restore register a6 */

#define ARM_SMCCC_VERSION_1_0		0x10000
#define ARM_SMCCC_VERSION_1_1		0x10001

#define ARM_SMCCC_VERSION_FUNC_ID					\
	ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL,				\
			   ARM_SMCCC_SMC_32,				\
			   0, 0)

#define ARM_SMCCC_STANDARD_SECURE_FUNC_ID					\
	ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL,				\
			   ARM_SMCCC_SMC_64,				\
			   4, 0)

#define ARM_SMCCC_STANDARD_HYPERVISOR_FUNC_ID					\
	ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL,				\
			   ARM_SMCCC_SMC_64,				\
			   5, 0)

/* Return codes defined in ARM DEN 0070A */
#define SMCCC_RET_SUCCESS			    0
#define SMCCC_RET_NOT_SUPPORTED			-1
#define SMCCC_RET_NOT_REQUIRED			-2


/*******************************************************************************
 * Bit definitions inside the function id as per the SMC calling convention
 ******************************************************************************/
#define FUNCID_TYPE_SHIFT		U(31)
#define FUNCID_TYPE_MASK		U(0x1)
#define FUNCID_TYPE_WIDTH		U(1)

#define FUNCID_CC_SHIFT			U(30)
#define FUNCID_CC_MASK			U(0x1)
#define FUNCID_CC_WIDTH			U(1)

#define FUNCID_OEN_SHIFT		U(24)
#define FUNCID_OEN_MASK			U(0x3f)
#define FUNCID_OEN_WIDTH		U(6)

#define FUNCID_NUM_SHIFT		U(0)
#define FUNCID_NUM_MASK			U(0xffff)
#define FUNCID_NUM_WIDTH		U(16)

#define GET_SMC_TYPE(id)		(((id) >> FUNCID_TYPE_SHIFT) & \
					 FUNCID_TYPE_MASK)
#define GET_SMC_CC(id)			(((id) >> FUNCID_CC_SHIFT) & \
					 FUNCID_CC_MASK)
#define GET_SMC_OEN(id)			(((id) >> FUNCID_OEN_SHIFT) & \
					 FUNCID_OEN_MASK)

/*******************************************************************************
 * Owning entity number definitions inside the function id as per the SMC
 * calling convention
 ******************************************************************************/
#define OEN_ARM_START			U(0)
#define OEN_ARM_END			U(0)
#define OEN_CPU_START			U(1)
#define OEN_CPU_END			U(1)
#define OEN_SIP_START			U(2)
#define OEN_SIP_END			U(2)
#define OEN_OEM_START			U(3)
#define OEN_OEM_END			U(3)
#define OEN_STD_START			U(4)	/* Standard Service Calls */
#define OEN_STD_END			U(4)
#define OEN_STD_HYP_START		U(5)	/* Standard Hypervisor Service calls */
#define OEN_STD_HYP_END			U(5)
#define OEN_VEN_HYP_START		U(6)	/* Vendor Hypervisor Service calls */
#define OEN_VEN_HYP_END			U(6)
#define OEN_TAP_START			U(48)	/* Trusted Applications */
#define OEN_TAP_END			U(49)
#define OEN_TOS_START			U(50)	/* Trusted OS */
#define OEN_TOS_END			U(63)
#define OEN_LIMIT			U(64)

/* Flags and error codes */
#define SMC_64				U(1)
#define SMC_32				U(0)

#define SMC_TYPE_FAST			ULL(1)
#define SMC_TYPE_YIELD			ULL(0)

#define SMC_OK				ULL(0)
#define SMC_UNK				-1
#define SMC_PREEMPTED			-2	/* Not defined by the SMCCC */
