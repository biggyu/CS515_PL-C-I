.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx

	subq $64, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq %rcx, -32(%rbp)
	movq %r8, -40(%rbp)
	movq %r9, -48(%rbp)
	movq $0, -56(%rbp)
	movq $0, -64(%rbp)
	jmp while0cond

while0cond:
	movq -8(%rbp), %rbx
	pushq %rbx
	movq -16(%rbp), %rbx
	pushq %rbx
	popq %rax
	popq %rbx
	cmpq %rax, %rbx
	jb while0body

	movq -56(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, %rax

	addq $64, %rsp
	popq %rbx
	popq %rbp
	ret
while0body:
	movq -24(%rbp), %rbx
	pushq %rbx
	movq -32(%rbp), %rbx
	pushq %rbx
	popq %rax
	popq %rbx
	cmpq %rax, %rbx
	je if7then

	movq -64(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $2, %r10
	pushq %r10
	popq %r10
	movq %r10, -64(%rbp)
	jmp if7end

if7then:
	movq -56(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $1, %r10
	pushq %r10
	popq %r10
	movq %r10, -56(%rbp)
	jmp if7end

if7end:
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $1, %r10
	pushq %r10
	popq %r10
	movq %r10, -8(%rbp)
	jmp while0cond
.section .note.GNU-stack,"",@progbits
