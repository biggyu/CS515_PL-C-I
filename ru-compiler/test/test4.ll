define i64 @foo(i64 %a) {
entry:
	%a.alloc = alloca i64
	%x.alloc = alloca i64
	store i64 %a, ptr %a.alloc
	store i64 0, ptr %x.alloc
	br label %while.cond

while.cond:
	%t2 = load i64, ptr %x.alloc
	%cmp1 = icmp ult i64 %t2, 5
	br i1 %cmp1, label %while.body, label %while.end

while.body:
	%t4 = load i64, ptr %x.alloc
	%t3 = add i64 %t4, 1
	store i64 %t3, ptr %x.alloc
	br label %while.cond

while.end:
	%t5 = load i64, ptr %x.alloc
	ret i64 %t5
}
