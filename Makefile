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

override CPP_LDFLAGS  ?= $(LDFLAGS)
# libcxxbridge1 contains cpp implementations for rust code that are required when compiling rust projects
# see: https://github.com/dtolnay/cxx/issues/875#issuecomment-913104697
CXX_HASH_DIR = $(shell find target/$(BUILD_MODE)/build/ -name "libcxxbridge1.a" -exec dirname {} \;)
override CPP_LDFLAGS  += -L${CXX_HASH_DIR} -l:libcxxbridge1.a

.PHONY: all build_cpp compile_cpp run_cpp clean

all: compile_cpp

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

clean:
	rm -f main
	cargo clean
