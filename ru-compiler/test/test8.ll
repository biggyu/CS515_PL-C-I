define i64 @foo(i64 %a) {
entry:
	%a.alloc = alloca i64
	%x.alloc = alloca i64
	store i64 %a, ptr %a.alloc
	store i64 42, ptr %x.alloc
	br i1 false, label %if.then, label %if.else

if.then:
	store i64 1, ptr %x.alloc
	br label %if.end

if.else:
	store i64 2, ptr %x.alloc
	br label %if.end

if.end:
	%t1 = load i64, ptr %x.alloc
	ret i64 %t1
}
