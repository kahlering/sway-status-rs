



all:
	gcc -c alsa.c -lalsa -Wl,--whole-archive, -lalsa
	ar rcs libalsa_test.a alsa.o

clean:
	rm alsa.o
	rm libalsa_test.a