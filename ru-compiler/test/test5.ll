define i64 @foo(i64 %a) {
entry:
	%a.alloc = alloca i64
	%b.alloc = alloca i64
	%c.alloc = alloca i64
	store i64 %a, ptr %a.alloc
	store i64 1, ptr %b.alloc
	br label %while.cond

while.cond:
	%t2 = load i64, ptr %b.alloc
	%cmp1 = icmp ult i64 %t2, 4
	br i1 %cmp1, label %while.body, label %while.end

while.body:
	%t4 = load i64, ptr %c.alloc
	%t5 = load i64, ptr %a.alloc
	%t3 = add i64 %t4, %t5
	store i64 %t3, ptr %c.alloc
	%t7 = load i64, ptr %b.alloc
	%t6 = add i64 %t7, 1
	store i64 %t6, ptr %b.alloc
	br label %while.cond

while.end:
	%t8 = load i64, ptr %c.alloc
	ret i64 %t8
}
