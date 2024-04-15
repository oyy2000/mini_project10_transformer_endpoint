toolchain:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustup --version			#rust toolchain manager

install:
	cargo clean &&\
		cargo build -j 1

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

release:
	cargo build --release

image:
	docker build -t rust-distro-cicd .

rundocker:
	docker run -p 8080:8080 rust-distro-cicd

watch:
	cargo lambda watch

build:
	cargo lambda build --release

deploy:
	cargo lambda deploy --region us-east-1

