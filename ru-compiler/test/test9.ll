define i64 @foo(i64 %a) {
entry:
	%a.alloc = alloca i64
	%x.alloc = alloca i64
	store i64 %a, ptr %a.alloc
	store i64 5, ptr %x.alloc
	%t2 = load i64, ptr %x.alloc
	%cmp1 = icmp ult i64 %t2, 10
	br i1 %cmp1, label %if.then, label %if.else

if.then:
	%t4 = load i64, ptr %a.alloc
	%t5 = load i64, ptr %x.alloc
	%cmp3 = icmp ugt i64 %t4, %t5
	br i1 %cmp3, label %if.then, label %if.else

if.then:
	store i64 %a, ptr %x.alloc
	br label %if.end

if.else:
	%t7 = load i64, ptr %x.alloc
	%t6 = add i64 %t7, 1
	store i64 %t6, ptr %x.alloc
	br label %if.end

if.end:
	br label %if.end

if.else:
	store i64 0, ptr %x.alloc
	br label %if.end

if.end:
	%t8 = load i64, ptr %x.alloc
	ret i64 %t8
}
