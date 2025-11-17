define i64 @foo(i64 %x) {
	%t1 = add i64 %x, 1
	%t2 = mul i64 5, %t1
	%t3 = add i64 42, %t2
	ret i64 %t3
}
