define i64 @foo(i64 %a) {
entry:
	%a.alloc = alloca i64
	%x.alloc = alloca i64
	store i64 %a, ptr %a.alloc
	store i64 3, ptr %x.alloc
	%t2 = load i64, ptr %a.alloc
	%t3 = load i64, ptr %x.alloc
	%cmp1 = icmp ult i64 %t2, %t3
	br i1 %cmp1, label %if.then, label %if.else

if.then:
	store i64 %a, ptr %x.alloc
	br label %if.end

if.else:
	%t5 = load i64, ptr %x.alloc
	%t4 = add i64 %t5, 1
	store i64 %t4, ptr %x.alloc
	br label %if.end

if.end:
	%t6 = load i64, ptr %x.alloc
	ret i64 %t6
}
