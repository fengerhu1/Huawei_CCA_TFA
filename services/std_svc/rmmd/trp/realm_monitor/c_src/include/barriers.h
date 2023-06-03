#ifndef __BARRIERS_H_
#define __BARRIERS_H_

#ifndef __ASSEMBLER__

#define compiler_barrier() asm volatile("" : : : "memory")

#define dsb(scope) asm volatile("dsb " #scope : : : "memory")
#define dmb(scope) asm volatile("dmb " #scope : : : "memory")

#define isb() asm volatile("isb" : : : "memory")

#define wfe()	asm volatile("wfe" : : : "memory")

#endif

#endif /* __BARRIERS_H_ */