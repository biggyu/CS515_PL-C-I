.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	addq %r9, %r8
	addq %r8, %rcx
	addq %rcx, %rdx
	addq %rdx, %rsi
	addq %rsi, %rdi
	movq %rdi, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
