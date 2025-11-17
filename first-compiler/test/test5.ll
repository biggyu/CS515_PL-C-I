define i64 @foo(i64 %x, i64 %y, i64 %z, i64 %x, i64 %y) {
	%t1 = add i64 %x, %y
	%t2 = add i64 %t1, %z
	%t3 = add i64 %t2, %x
	%t4 = add i64 %t3, %y
	ret i64 %t4
}
