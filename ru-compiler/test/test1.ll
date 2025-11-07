define i64 @foo(i64 %a) {
entry:
	%a.alloc = alloca i64
	%x.alloc = alloca i64
	store i64 %a, ptr %a.alloc
	store i64 5, ptr %x.alloc
	br i1 true, label %if.then, label %if.else

if.then:
	%t2 = load i64, ptr %x.alloc
	%t1 = add i64 %t2, 1
	store i64 %t1, ptr %x.alloc
	br label %if.end

if.else:
	%t4 = load i64, ptr %x.alloc
	%t3 = add i64 %t4, 2
	store i64 %t3, ptr %x.alloc
	br label %if.end

if.end:
	%t5 = load i64, ptr %x.alloc
	ret i64 %t5
}
