files = panicker out-of-bounds recover \
	recover-new recover-newer recover-newest \
	recover-newester recover-newestest recover-newestester

.PHONY: build clean $(files)

build: $(files) clean

$(files):
	rustc -o $@.exe $@.rs
	-./$@.exe

clean:
	-rm *.exe
	-rm *.pdb