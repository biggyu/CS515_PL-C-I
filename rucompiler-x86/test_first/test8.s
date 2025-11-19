.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	addq $1, %rdi
	addq $2, %rsi
	imulq %rsi, %rdi
	addq $3, %rdx
	addq $4, %rcx
	imulq %rcx, %rdx
	addq %rdx, %rdi
	addq $5, %rdi
	movq %rdi, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
