define i64 @foo(i64 %a) {
entry:
	%a.alloc = alloca i64
	%x.alloc = alloca i64
	store i64 %a, ptr %a.alloc
	store i64 10, ptr %x.alloc
	%t2 = load i64, ptr %x.alloc
	%cmp1 = icmp uge i64 %t2, 3
	br i1 %cmp1, label %if.then, label %if.else

if.then:
	%t4 = load i64, ptr %x.alloc
	%t5 = load i64, ptr %a.alloc
	%t3 = add i64 %t4, %t5
	store i64 %t3, ptr %x.alloc
	br label %if.end

if.else:
	%t7 = load i64, ptr %x.alloc
	%t8 = load i64, ptr %a.alloc
	%t6 = mul i64 %t7, %t8
	store i64 %t6, ptr %x.alloc
	br label %if.end

if.end:
	%t9 = load i64, ptr %x.alloc
	ret i64 %t9
}
