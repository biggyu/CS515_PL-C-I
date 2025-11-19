.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	imulq $6, %rdi
	addq $5, %rdi
	imulq %rsi, %rdi
	addq $4, %rdi
	imulq %rdx, %rdi
	addq $3, %rdi
	movq %rdi, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
