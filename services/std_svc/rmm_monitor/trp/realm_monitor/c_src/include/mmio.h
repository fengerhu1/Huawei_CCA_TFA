/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */
#ifndef __MMIO_H_
#define __MMIO_H_

#include <barriers.h>
#include <stdint.h>

static inline uint8_t read8(volatile void *addr)
{
	uint8_t val;

	dsb(ld);
	asm volatile("ldrb %w0, [%1]": "=r" (val) : "r" (addr));
	dsb(ld);
	return val;
}

static inline void write8(uint8_t val, volatile void *addr)
{
	dsb(st);
	asm volatile("strb %w0, [%1]":  : "r" (val), "r" (addr));
	dsb(st);
}

static inline uint16_t read16(volatile void *addr)
{
	uint16_t val;

	dsb(ld);
	asm volatile("ldrh %w0, [%1]": "=r" (val) : "r" (addr));
	dsb(ld);
	return val;
}

static inline void write16(uint16_t val, volatile void *addr)
{
	dsb(st);
	asm volatile("strh %w0, [%1]":  : "r" (val), "r" (addr));
	dsb(st);
}

static inline uint32_t read32(volatile void *addr)
{
	uint32_t val;

	dsb(ld);
	asm volatile("ldr %w0, [%1]": "=r" (val) : "r" (addr));
	dsb(ld);
	return val;
}

static inline void write32(uint32_t val, volatile void *addr)
{
	dsb(st);
	asm volatile("str %w0, [%1]":  : "r" (val), "r" (addr));
	dsb(st);
}

static inline uint64_t read64(volatile void *addr)
{
	uint64_t val;

	dsb(ld);
	asm volatile("ldr %0, [%1]": "=r" (val) : "r" (addr));
	dsb(ld);
	return val;
}

static inline void write64(uint64_t val, volatile void *addr)
{
	dsb(st);
	asm volatile("str %0, [%1]":  : "r" (val), "r" (addr));
	dsb(st);
}

#endif /* __MMIO_H_ */
