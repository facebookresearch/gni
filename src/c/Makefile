
.PHONY: all libGNI.a clean

all: libGNI.a

libGNI.a: GNI.o
	ar rcs $@ $^

%.o: %.c
	gcc -Wall -Wextra -O2 -fPIC -c $< -o $@

clean:
	rm -f *.o libGNI.a
