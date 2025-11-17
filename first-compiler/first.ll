define i64 @foo(i64 %c, i64 %e, i64 %b) {
	%t1 = mul i64 %e, %b
	%t2 = add i64 %c, %t1
	ret i64 %t2
}
