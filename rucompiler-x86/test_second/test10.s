.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx

	subq $32, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq $0, -24(%rbp)
	movq $1, -32(%rbp)
	movq -8(%rbp), %rbx
	pushq %rbx
	movq -16(%rbp), %rbx
	pushq %rbx
	popq %rax
	popq %r10
	mulq %r10
	pushq %rax
	movq -32(%rbp), %rbx
	pushq %rbx
	movq -24(%rbp), %rbx
	pushq %rbx
	popq %rax
	popq %r10
	mulq %r10
	pushq %rax
	popq %r10
	popq %r11
	addq %r10, %r11
	pushq %r11
	popq %r11
	movq %r11, -24(%rbp)
	movq $6, %r10
	pushq %r10
	popq %r10
	addq $1, %r10
	pushq %r10
	popq %r10
	movq %r10, -32(%rbp)
	movq -32(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $2, %r10
	pushq %r10
	movq -24(%rbp), %rbx
	pushq %rbx
	popq %rax
	popq %r10
	mulq %r10
	pushq %rax
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %r10
	imulq $2, %r10, %r10
	pushq %r10
	popq %r10
	popq %r11
	addq %r10, %r11
	pushq %r11
	popq %r11
	movq %r11, -16(%rbp)
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %rax
	movq $4, %rbx
	cmpq %rbx, %rax
	jb if18then

	movq $6, -24(%rbp)
	movq -24(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $2, %r10
	pushq %r10
	popq %r10
	movq %r10, -32(%rbp)
	jmp if18end

if18then:
	movq $4, -24(%rbp)
	movq -24(%rbp), %rbx
	pushq %rbx
	movq -32(%rbp), %rbx
	pushq %rbx
	popq %rax
	popq %r10
	mulq %r10
	pushq %rax
	popq %rax
	movq %rax, -32(%rbp)
	jmp if18end

if18end:

	movq $10, -24(%rbp)
	jmp if24end

if24then:
	movq $8, -24(%rbp)
	jmp if24end

if24end:
	jmp while25cond

while25cond:
	movq -8(%rbp), %rbx
	pushq %rbx
	movq $20, %rax
	popq %rbx
	cmpq %rbx, %rax
	ja while25body

	movq -32(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, %rax

	addq $32, %rsp
	popq %rbx
	popq %rbp
	ret
while25body:
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $4, %r10
	pushq %r10
	popq %r10
	movq %r10, -8(%rbp)
	movq -24(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $1, %r10
	pushq %r10
	popq %r10
	movq %r10, -24(%rbp)
	jmp while25cond
.section .note.GNU-stack,"",@progbits
