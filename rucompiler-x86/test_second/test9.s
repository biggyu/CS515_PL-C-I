.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx

	subq $16, %rsp
	movq %rdi, -8(%rbp)
	movq $5, -16(%rbp)
	movq -16(%rbp), %rbx
	pushq %rbx
	popq %rbx
	cmpq %rbx, $10
	jb if2then

	movq -16(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $1, %r10
	pushq %r10
	popq %r10
	movq %r10, -16(%rbp)
	jmp if6end

if6then:
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, -16(%rbp)

if6end:
	jmp if2end

	movq $0, -16(%rbp)
	jmp if2end

if6then:
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, -16(%rbp)

if2end:
	movq -16(%rbp), %rbx
	pushq %rbx
	movq %rbx, %rax

	addq $16, %rsp
	popq %rbx
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
