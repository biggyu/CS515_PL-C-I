define i64 @foo(i64 %a) {
entry:
	%a.alloc = alloca i64
	%x.alloc = alloca i64
	store i64 %a, ptr %a.alloc
	%t1 = mul i64 3, 5
	%t2 = add i64 2, %t1
	store i64 %t2, ptr %x.alloc
	%t4 = load i64, ptr %x.alloc
	%cmp3 = icmp eq i64 %t4, 17
	br i1 %cmp3, label %if.then, label %if.else

if.then:
	%t6 = load i64, ptr %x.alloc
	%t5 = add i64 %t6, 1
	store i64 %t5, ptr %x.alloc
	br label %if.end

if.else:
	%t8 = load i64, ptr %x.alloc
	%t7 = mul i64 %t8, 2
	store i64 %t7, ptr %x.alloc
	br label %if.end

if.end:
	%t9 = load i64, ptr %x.alloc
	ret i64 %t9
}
