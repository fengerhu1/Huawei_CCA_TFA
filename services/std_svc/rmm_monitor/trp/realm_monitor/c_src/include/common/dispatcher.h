#pragma once
#include <stdint.h>

struct titanium_handler_table {
	uint32_t std_smc_entry;
	uint32_t fast_smc_entry;
	uint32_t fiq_entry;
};
extern struct titanium_handler_table titanium_handler_table;
