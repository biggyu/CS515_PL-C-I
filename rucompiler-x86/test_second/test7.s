.text
	.global foo

foo:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx

	subq $16, %rsp
	movq %rdi, -8(%rbp)
	movq $0, -16(%rbp)
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %rax
	movq $0, %rbx
	cmpq %rbx, %rax
	ja if2then

	movq $100, -16(%rbp)
	jmp if2end

if2then:
	jmp while3cond

while3cond:
	movq -16(%rbp), %rbx
	pushq %rbx
	movq -8(%rbp), %rbx
	pushq %rbx
	popq %rax
	popq %rbx
	cmpq %rax, %rbx
	jb while3body

	jmp if2end
while3body:
	movq -16(%rbp), %rbx
	pushq %rbx
	popq %r10
	addq $1, %r10
	pushq %r10
	popq %r10
	movq %r10, -16(%rbp)
	jmp while3cond

if2end:
	movq -16(%rbp), %rbx
	pushq %rbx
	popq %rbx
	movq %rbx, %rax

	addq $16, %rsp
	popq %rbx
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
