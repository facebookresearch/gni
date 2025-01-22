# Copyright (c) Meta Platforms, Inc. and affiliates.
# All rights reserved.

# This source code is licensed under the license found in the
# LICENSE file in the root directory of this source tree.

.PHONY: all libGNI.a clean

all: libGNI.a

libGNI.a: GNI.o
	ar rcs $@ $^

%.o: %.c
	gcc -Wall -Wextra -O2 -fPIC -c $< -o $@

clean:
	rm -f *.o libGNI.a
