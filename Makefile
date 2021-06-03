export:
	cp ./rust_factorial/target/release/librust_factorial.rlib ./swift_calling/Sources/CFactorial/
	cp ./rust_factorial/CFactorial.h                          ./swift_calling/Sources/CFactorial/
