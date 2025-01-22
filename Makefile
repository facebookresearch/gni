# Copyright (c) Meta Platforms, Inc. and affiliates.
# All rights reserved.

# This source code is licensed under the license found in the
# LICENSE file in the root directory of this source tree.

BUILD_MODE ?= debug

ifeq ($(BUILD_MODE), release)
    CARGO_FLAG := --release
else
    CARGO_FLAG :=
endif

override CPP_FLAGS  ?=
override CPP_FLAGS  += -std=c++17
override CPP_FLAGS  += -Itarget/cxxbridge/gni/src/cpp
override CPP_FLAGS  += -Itarget/cxxbridge/rust

override CPP_SRC  ?=
override CPP_SRC  += target/cxxbridge/gni/src/cpp/mod.rs.cc
override CPP_SRC  += src/cpp/GNI.cpp
override CPP_SRC  += src/cpp/main.cpp

override LDFLAGS  ?=
override LDFLAGS  += -Ltarget/$(BUILD_MODE) -lgni_lib

override CPP_LDFLAGS  ?= $(LDFLAGS)
# libcxxbridge1 contains cpp implementations for rust code that are required when compiling rust projects
# see: https://github.com/dtolnay/cxx/issues/875#issuecomment-913104697
CXX_HASH_DIR = $(shell find target/$(BUILD_MODE)/build/ -name "libcxxbridge1.a" -exec dirname {} \;)
override CPP_LDFLAGS  += -L${CXX_HASH_DIR} -l:libcxxbridge1.a

override C_SRC  ?=
override C_SRC  += src/c/GNI.c
override C_SRC  += src/c/main.c

.PHONY: all build_cpp compile_cpp run_cpp build_c compile_c run_c build_go run_go clean

all: compile_cpp compile_c build_go

# ------------------------------------------------------------
# C++ Targets
# ------------------------------------------------------------

build_cpp:
	cargo build --features "cpp" $(CARGO_FLAG)

compile_cpp: build_cpp
	g++ \
    ${CPP_FLAGS} \
    ${CPP_SRC} \
    ${CPP_LDFLAGS} \
    -o main_cpp

run_cpp: compile_cpp
	LD_LIBRARY_PATH=./target/$(BUILD_MODE):$$LD_LIBRARY_PATH ./main_cpp

# ------------------------------------------------------------
# C Targets
# ------------------------------------------------------------

build_c:
	cargo build --features "c" $(CARGO_FLAG)

compile_c: build_c
	gcc \
    ${C_SRC} \
    ${LDFLAGS} \
    -o main_c

run_c: compile_c
	LD_LIBRARY_PATH=./target/$(BUILD_MODE):$$LD_LIBRARY_PATH ./main_c

# ------------------------------------------------------------
# Go Targets
# ------------------------------------------------------------

build_go: build_c
	cd src/c && make
	cd src/go && go build

run_go: build_go
	LD_LIBRARY_PATH=./target/$(BUILD_MODE):$$LD_LIBRARY_PATH ./src/go/go

clean:
	rm -f main_c
	rm -f main_cpp
	cd src/go && go clean
	cd src/c && make clean
	cargo clean
