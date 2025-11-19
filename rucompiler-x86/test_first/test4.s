.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	imulq $2, %r9
	addq %r9, %r8
	imulq %r8, %rcx
	addq %rcx, %rdx
	imulq %rdx, %rsi
	addq %rsi, %rdi
	movq %rdi, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
