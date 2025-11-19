.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	movq $1, %rax
	imulq $2, %rax
	pushq %rax
	movq $3, %rax
	addq $5, %rax
	pushq %rax
	popq %r10
	popq %rax
	addq %rax, %r10
	pushq %r10
	movq $4, %rax
	addq $6, %rax
	pushq %rax
	movq $5, %rax
	addq $7, %rax
	pushq %rax
	popq %r10
	popq %rax
	imulq %rax, %r10
	pushq %r10
	popq %r10
	popq %rax
	imulq %r10, %r10
	pushq %r10
	popq %r10
	addq $6, %r10
	movq %r10, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
