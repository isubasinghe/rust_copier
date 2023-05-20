.option norvc
.section .data
# boot from OpenSBI
# a0 current hart id 
# a1 pointer to flattened device tree

.section .text.init
.global _start 
_start:
	bnez a0, 3f 
	csrw satp, zero
.option push 
.option norelax
	la gp, _global_pointer
.option pop
	la a2, _bss_start
	la a3, _bss_end
	bgeu a2, a3, 2f
1:
	sd zero, (a2)
	addi a2, a2, 8
	bltu a2, a3, 1b
2:	
	la sp, _stack_end
	j 2b
3:
	wfi 
	j 3b
