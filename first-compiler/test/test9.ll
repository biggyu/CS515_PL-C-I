define i64 @foo(i64 %a, i64 %b, i64 %c, i64 %b, i64 %c, i64 %a, i64 %b, i64 %c, i64 %a, i64 %a) {
	%t1 = add i64 %a, 2
	%t2 = add i64 %b, %c
	%t3 = mul i64 %t2, 2
	%t4 = add i64 %t2, %t3
	%t5 = mul i64 %t1, %t4
	%t6 = mul i64 %a, 3
	%t7 = mul i64 %t2, %t1
	%t8 = add i64 %t6, %t7
	%t9 = add i64 %t5, %t8
	%t10 = add i64 %t9, %t1
	ret i64 %t10
}
