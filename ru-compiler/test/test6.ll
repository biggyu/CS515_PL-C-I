define i64 @foo(i64 %a) {
entry:
	%a.alloc = alloca i64
	%x.alloc = alloca i64
	%y.alloc = alloca i64
	store i64 %a, ptr %a.alloc
	store i64 0, ptr %x.alloc
	store i64 1, ptr %y.alloc
	br label %while.cond

while.cond:
	%t2 = load i64, ptr %x.alloc
	%cmp1 = icmp ult i64 %t2, 3
	br i1 %cmp1, label %while.body, label %while.end

while.body:
	%t4 = load i64, ptr %a.alloc
	%t3 = mul i64 %t4, 2
	store i64 %t3, ptr %y.alloc
	%t6 = load i64, ptr %x.alloc
	%t5 = add i64 %t6, 1
	store i64 %t5, ptr %x.alloc
	br label %while.cond

while.end:
	%t7 = load i64, ptr %y.alloc
	ret i64 %t7
}
