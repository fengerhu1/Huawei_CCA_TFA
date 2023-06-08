/*
 * Copyright (c) 2021-2022, Arm Limited and Contributors. All rights reserved.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#ifndef PLATFORM_TRP_H
#define PLATFORM_TRP_H

#include <services/rmm_core_manifest.h>

struct rmm_manifest;

/*******************************************************************************
 * Mandatory TRP functions (only if platform contains a TRP)
 ******************************************************************************/
void trp_early_platform_setup(struct rmm_manifest *manifest);
void trp_early_platform_setup_qemu();
void trp_plat_arch_setup(void);
void trp_plat_arch_enable_mmu(int linearId);

#endif /* PLATFORM_TRP_H */
