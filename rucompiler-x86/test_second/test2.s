.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx

	subq $16, %rsp
	movq %rdi, -8(%rbp)
	movq $2, %r11
	mulq $3, %r11
	pushq %r11
	popq %r11
	addq $1, %r11
	pushq %r11
	movq $6, %r11
	addq $7, %r11
	pushq %r11
	popq %r11
	addq $5, %r11
	pushq %r11
	popq %r11
	mulq $4, %r11
	pushq %r11
	popq %r10
	popq %r11
	addq %r10, %r11
	pushq %r11
	popq %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %rbx
	pushq %rbx
	movq %rbx, %rax

	addq $16, %rsp
	popq %rbx
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
