define i64 @foo(i64 %a, i64 %x) {
entry:
	%a.alloca = alloca i64
	%x.alloca = alloca i64
	%b.alloca = alloca i64
	%c.alloca = alloca i64
	store i64 %a, ptr %a.alloc
	store i64 %x, ptr %x.alloc
	store i64 0, ptr %b.alloc
	store i64 1, ptr %c.alloc
	%cmp3 = icmp ult i64 %a, 4
	br i1 %cmp3 label %if.then, label %if.else

if.then:
	store i64 4, ptr %b.alloc
	%t6 = mul i64 %b, %c
	store i64 %t6, ptr %c.alloc
	br label %if.end

if.else:
	store i64 6, ptr %b.alloc
	%t8 = add i64 %b, 2
	store i64 %t8, ptr %c.alloc
	br label %if.end

if.end:
	br i1 false label %if.then, label %if.else

if.then:
	store i64 8, ptr %b.alloc
	br label %if.end

if.else:
	store i64 10, ptr %b.alloc
	br label %if.end

if.end:
	br label %while.cond

while.cond:
	%cmp10 = icmp ugt i64 20, %a
	br i1 %cmp10, label %while.body, label %while.end

while.body:
	%t11 = add i64 %a, 4
	store i64 %t11, ptr %a.alloc
	%t13 = add i64 %b, 1
	store i64 %t13, ptr %b.alloc
	br label %while.cond

while.end:
	%t5 = load i64, ptr %c.alloc
	ret i64 %t5
}
