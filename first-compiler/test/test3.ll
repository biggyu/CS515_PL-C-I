define i64 @foo(i64 %i, i64 %j, i64 %i, i64 %j) {
	%t1 = mul i64 %i, %j
	%t2 = add i64 %t1, %t1
	ret i64 %t2
}
