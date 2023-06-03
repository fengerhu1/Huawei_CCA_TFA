#ifndef __MEMORY_H_
#define __MEMORY_H_

#include <stddef.h>
#include <stdint.h>

/* Single-Copy Atomic 64-bit write */
static inline void __sca_write64(uint64_t *ptr, uint64_t val)
{
	asm volatile(
	"	str %[val], %[ptr]\n"
	: [ptr] "=m" (*ptr)
	: [val] "r" (val)
	);
}
#define SCA_WRITE64(_p, _v) __sca_write64((void *)(_p), ((uint64_t)(_v)))

/* Single-Copy Atomic 64-bit read */
static inline uint64_t __sca_read64(uint64_t *ptr)
{
	uint64_t val;

	asm volatile(
	"	ldr	%[val], %[ptr]\n"
	: [val] "=r" (val)
	: [ptr] "m" (*ptr)
	);

	return val;
}
#define SCA_READ64(_p) ((typeof(*(_p)))__sca_read64((void *)(_p)))

/* Single-Copy Atomic 64-bit read with ACQUIRE memory ordering semantics */
static inline uint64_t __sca_read64_acquire(uint64_t *ptr)
{
	uint64_t val;

	asm volatile(
	"	ldar	%[val], %[ptr]\n"
	: [val] "=r" (val)
	: [ptr] "Q" (*ptr)
	);

	return val;
}
#define SCA_READ64_ACQUIRE(_p) ((typeof(*(_p)))__sca_read64_acquire((void *)(_p)))

#endif /* __MEMORY_H_ */
