#
# Copyright (c) 2021-2023 Arm Limited and Contributors. All rights reserved.
#
# SPDX-License-Identifier: BSD-3-Clause
#

RMM_SOURCES		+=	services/std_svc/rmm_monitor/trp/trp_entry.S \
				services/std_svc/rmm_monitor/trp/run-asm.S \
				services/std_svc/rmm_monitor/trp/vectors.S \
				services/std_svc/rmm_monitor/trp/trp_main.c  \
				services/std_svc/rmm_monitor/trp/trp_helpers.c

RMM_DEFAULT_LINKER_SCRIPT_SOURCE := services/std_svc/rmm_monitor/trp/linker.ld.S

RMM_LIBS		:= 	services/std_svc/rmm_monitor/trp/realm_monitor/target/aarch64-unknown-none-softfloat/release/librealm_monitor.a \
					services/std_svc/rmm_monitor/trp/realm_monitor/libc_rmm.a
					# services/../../mbedtls/library/libmbedcrypto.a

RMM_CPPFLAGS   +=      -DPLAT_XLAT_TABLES_DYNAMIC

# Include the platform-specific TRP Makefile
# If no platform-specific TRP Makefile exists, it means TRP is not supported
# on this platform.
TRP_PLAT_MAKEFILE := $(wildcard ${PLAT_DIR}/trp/trp-${PLAT}.mk)
ifeq (,${TRP_PLAT_MAKEFILE})
  $(error TRP is not supported on platform ${PLAT})
else
  include ${TRP_PLAT_MAKEFILE}
endif
