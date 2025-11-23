.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx

	subq $16, %rsp
	movq %rdi, -8(%rbp)
	movq $10, -16(%rbp)
	jmp if0else

	jmp if0end

if0then:

if0end:
	jmp while1cond

while1cond:
	jmp while1end

	movq -16(%rbp), %rbx
	pushq %rbx
	movq %rbx, %rax

	addq $16, %rsp
	popq %rbx
	popq %rbp
	ret

while1body:
	jmp while1cond
.section .note.GNU-stack,"",@progbits
