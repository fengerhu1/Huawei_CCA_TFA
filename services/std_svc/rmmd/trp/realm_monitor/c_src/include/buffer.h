#ifndef __BUFFER_H_
#define __BUFFER_H_

enum BufferSlot {
	/*
	 * NS or Realm-private.
	 */
	SLOT_NS,
	SLOT_INPUT,
	SLOT_OUTPUT,

	/*
	 * RMM-private.
	 */
	SLOT_DELEGATED,
	SLOT_RD,
	SLOT_REC,
	SLOT_REC_TARGET,	/* Target REC for interrupts */
	SLOT_RTT,
	SLOT_RTT2,
	SLOT_REC_LIST,
	NR_CPU_SLOTS
};
#endif
