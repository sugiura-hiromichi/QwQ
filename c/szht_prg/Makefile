file=$${NEOVIM_CXX_AUTO_RUNNED_FILE}

run: a.out
	./a.out

./a.out: $(file)
	rm -f a.out
	clang -o .a/out $(file)

clean:
	rm -f a.out
