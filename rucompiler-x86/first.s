.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	imulq %rdx, %rsi
	addq %rsi, %rdi
	movq %rdi, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
