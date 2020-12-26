SRCS = step0_repl step1_read_print stepA_mal

.PHONY: build
all:
	cd ./mal/src;\
	cargo build

$(SRCS):
	cd ./mal/src;\
	cargo build --bin $(patsubst %.rs,%,$(filter step%,$(@)))

RUN:= run
STEP:= step
.PHONY: run
run: clean all
	cd ./mal/src;\
	cargo run --bin ($(subst $(RUN),$(STEP),$@)_repl)

.PHONY: clean
clean:
	cd ./mal/src;\
	cargo clean