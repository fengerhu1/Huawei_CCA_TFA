#ifndef __SIZES_H_
#define __SIZES_H_

#include <const.h>

#define SZ_1K	(UL(1) << 10)
#define SZ_1M	(UL(1) << 20)
#define SZ_1G	(UL(1) << 30)

#define SZ_4K	(4  * SZ_1K)
#define SZ_16K	(16 * SZ_1K)
#define SZ_64K	(64 * SZ_1K)

#define SZ_2G	(2  * SZ_1G)
#define SZ_2M	(2  * SZ_1M)

#endif /* __SIZES_H_ */
