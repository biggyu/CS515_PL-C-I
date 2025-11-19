.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	addq $2, %rdx
	imulq $10, %rdx
	addq %rdx, %rsi
	imulq %rsi, %rdi
	addq $1, %rdi
	movq %rdi, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
