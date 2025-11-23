.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx

	subq $40, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq $1, -24(%rbp)
	movq $2, -32(%rbp)
	movq $3, -40(%rbp)
	movq -8(%rbp), %rbx
	movq -16(%rbp), %rax
	pushq %rax
	pushq %rbx
	popq %rbx
	popq %rax
	cmpq %rbx, %rax
	jb if3then

while4cond:
	movq -24(%rbp), %rbx
	pushq %rbx
	popq %rbx
	cmpq %rbx, $5
	jb while4body

	jmp if3end

while4body:
	movq -24(%rbp), %rbx
	pushq %rbx
	popq %r10
	mulq $2, %r10
	pushq %r10
	movq -32(%rbp), %rbx
	pushq %rbx
	popq %r10
	popq %r11
	addq %r10, %r11
	pushq %r11
	popq %r11
	movq %r11, -32(%rbp)
	movq -24(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $1, %r10
	pushq %r10
	popq %r10
	movq %r10, -24(%rbp)
	jmp while4cond

while13cond:
	movq -32(%rbp), %rbx
	pushq %rbx
	popq %rbx
	cmpq %rbx, $10
	jb while13body

	jmp if3end

while13body:
	movq -24(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $1, %r10
	pushq %r10
	movq -32(%rbp), %rbx
	pushq %rbx
	popq %r10
	popq %r11
	mulq %r10, %r11
	pushq %r11
	movq -40(%rbp), %rbx
	pushq %rbx
	popq %r10
	popq %r11
	addq %r10, %r11
	pushq %r11
	popq %r11
	movq %r11, -40(%rbp)
	movq -32(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $1, %r10
	pushq %r10
	popq %r10
	movq %r10, -32(%rbp)
	jmp while13cond

	jmp while13cond

if3then:

if3end:
	movq -32(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $2, %r10
	pushq %r10
	movq -40(%rbp), %rbx
	pushq %rbx
	popq %r10
	popq %r11
	mulq %r10, %r11
	pushq %r11
	movq -24(%rbp), %rbx
	pushq %rbx
	popq %r10
	popq %r11
	addq %r10, %r11
	pushq %r11
	popq %r11
	movq %r11, -24(%rbp)
	movq -24(%rbp), %rbx
	pushq %rbx
	movq %rbx, %rax

	addq $40, %rsp
	popq %rbx
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
