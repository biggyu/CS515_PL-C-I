define i64 @foo() {
	%t1 = add i64 4, 2
	%t2 = mul i64 %t1, 2
	%t3 = add i64 %t2, 1
	ret i64 %t3
}
