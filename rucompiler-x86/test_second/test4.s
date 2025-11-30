.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx

	subq $32, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -8(%rbp), %rbx
	pushq %rbx
	movq -16(%rbp), %rbx
	pushq %rbx
	popq %rax
	popq %rbx
	cmpq %rax, %rbx
	jb if3then

	movq $0, -24(%rbp)
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, -32(%rbp)
	movq -16(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, -8(%rbp)
	movq -32(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, -16(%rbp)
	jmp if3end

if3then:
	movq $0, -24(%rbp)
	jmp if3end

if3end:
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, %rax

	addq $32, %rsp
	popq %rbx
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
