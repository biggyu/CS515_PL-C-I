.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	addq %rsi, %rdi
	addq %rcx, %rdx
	imulq %rdx, %rdi
	addq %r9, %r8
	addq %r8, %rdi
	movq %rdi, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
