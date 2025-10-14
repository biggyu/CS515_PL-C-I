define i64 @foo(i64 %b, i64 %a, i64 %c) {
	%t1 = add i64 %a, %b
	%t2 = mul i64 %t1, %c
	ret i64 %t2
}
