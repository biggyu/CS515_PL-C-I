define i64 @foo(i64 %a, i64 %b, i64 %c) {
	%t1 = add i64 %a, 2
	%t2 = add i64 %b, %c
	%t3 = mul i64 %t2, 2
	%t4 = add i64 %t2, %t3
	%t5 = mul i64 %t1, %t4
	%t6 = mul i64 %a, 3
	%t7 = add i64 %t5, %t6
	ret i64 %t7
}
