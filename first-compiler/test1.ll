define i64 @foo(i64 %a, i64 %b, i64 %c) {
	%t1 = add i64 %b, %c
	%t2 = mul i64 2, %t1
	%t3 = add i64 %a, %t2
	%t4 = mul i64 %t1, %a
	%t5 = add i64 %t3, %t4
	ret i64 %t5
}
