.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx

	subq $16, %rsp
	movq %rdi, -8(%rbp)
	movq $0, -16(%rbp)

	movq -16(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $2, %r10
	pushq %r10
	popq %r10
	movq %r10, -16(%rbp)
	jmp if0end

if0then:
	movq -16(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $1, %r10
	pushq %r10
	popq %r10
	movq %r10, -16(%rbp)
	jmp if0end

if0end:
	movq -16(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, %rax

	addq $16, %rsp
	popq %rbx
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
