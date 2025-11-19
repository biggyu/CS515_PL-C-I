.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	imulq %rsi, %rdi
	imulq %rdx, %rdi
	imulq %rcx, %rdi
	imulq %r8, %rdi
	imulq %r9, %rdi
	movq %rdi, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
