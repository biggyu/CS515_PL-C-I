.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	addq $1, %rdi
	imulq %rsi, %rdi
	addq %rdx, %rdi
	imulq %rcx, %rdi
	addq $42, %rdi
	movq %rdi, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
