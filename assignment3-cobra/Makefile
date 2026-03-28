# Keep generated assembly files
.PRECIOUS: test/%.s

test/%.s: test/%.snek src/main.rs
	cargo run -- $< test/$*.s

test/%.run: test/%.s runtime/start.rs
	nasm -f macho64 test/$*.s -o runtime/our_code.o
	ar rcs runtime/libour_code.a runtime/our_code.o
	rustc -L runtime/ runtime/start.rs -o test/$*.run

test: \
	test/num.run \
	test/true_val.run \
	test/false_val.run \
	test/input_num.run \
	test/input_true.run \
	test/add1.run \
	test/sub1.run \
	test/negate.run \
	test/isnum_num.run \
	test/isnum_bool.run \
	test/isbool_num.run \
	test/isbool_bool.run \
	test/if_true.run \
	test/if_false.run \
	test/block.run \
	test/let_set.run \
	test/loop_break.run \
	test/eq_num.run \
	test/lt_num.run \
	test/ge_num.run

	./test/num.run
	./test/true_val.run
	./test/false_val.run
	./test/input_num.run 7
	./test/input_true.run true
	./test/add1.run
	./test/sub1.run
	./test/negate.run
	./test/isnum_num.run
	./test/isnum_bool.run
	./test/isbool_num.run
	./test/isbool_bool.run
	./test/if_true.run
	./test/if_false.run
	./test/block.run
	./test/let_set.run
	./test/loop_break.run
	./test/eq_num.run
	./test/lt_num.run
	./test/ge_num.run
