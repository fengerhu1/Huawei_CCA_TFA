#ifndef __ASSERT_H_
#define __ASSERT_H_

#include <assembler.h>
#include <barriers.h>
#include <debug.h>

extern void print_info(const char *fmt, ...);

#define assert(_cond) 							\
	do {								\
		if (!(_cond)) {						\
			print_info("Assertion failed %s:%d\n",		\
				 __FILE__, __LINE__);			\
			while (1)					\
				wfe();					\
		}							\
	} while (0)

#define COMPILER_ASSERT(_condition) extern char compiler_assert[(_condition) ? 1 : -1]

/*
 * If _expr is false, this will result in a compile time error as it tries to
 * define a bitfield of size -1 in that case.  Otherwise, it will define a
 * bitfield of size 0, which is valid, and not create a compiler warning.
 *
 * The return value is only relevant when the compilation succeeds, and by
 * subtracting the size of the same struct, this should always return 0 as a
 * value and can be included in other expressions.
 */
#define COMPILER_ASSERT_ZERO(_expr) (sizeof(struct { char : (-!(_expr)); }) \
				- sizeof(struct { char : 0; }))

#define CHECK_TYPE_IS_ARRAY(_v) \
	COMPILER_ASSERT_ZERO(!__builtin_types_compatible_p(typeof(_v), typeof(&(_v[0]))))

#endif /* __ASSERT_H_ */
