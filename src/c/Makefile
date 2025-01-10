
all: libGNI.a clean

libGNI.a: GNI.o
	ar rcs $@ $^

%.o: %.c
	gcc -Wall -Wextra -O2 -fPIC -c $< -o $@

clean:
	rm -f *.o libGNI.a
