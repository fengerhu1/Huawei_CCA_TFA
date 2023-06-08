#ifndef __CONST_H_
#define __CONST_H_

#if defined(__ASSEMBLER__) || defined(__LINKER__)
#define UL(x)	(x)
#else
#define UL(x)	(x##UL)
#endif

#endif /* __CONST_H_ */
