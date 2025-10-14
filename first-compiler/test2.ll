define i64 @foo(i64 %c, i64 %a, i64 %b) {
	%t1 = mul i64 %b, %c
	%t2 = add i64 %a, %t1
	ret i64 %t2
}
