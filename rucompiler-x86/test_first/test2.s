.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	movq $2, %rax
	imulq $3, %rax
	pushq %rax
	popq %rax
	addq $1, %rax
	pushq %rax
	movq $4, %rax
	imulq $5, %rax
	pushq %rax
	popq %r10
	popq %rax
	addq %rax, %r10
	pushq %r10
	popq %r10
	addq $6, %r10
	movq %r10, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
