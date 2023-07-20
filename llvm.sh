# note that this just checks if clang compiles the LLVM, not if the output is correct!

files=$(find ./program_examples -type f | sort)

for file in $files
do
	cargo run -- --llvm-print $file > ./out.ll 2> /dev/null
	clang ./out.ll >/dev/null 2>&1
	status=$(echo $?)
	echo "${file}: ${status}"
	rm -f ./out.ll
done

rm -f ./a.out
