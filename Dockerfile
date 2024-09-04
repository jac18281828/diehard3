FROM ghcr.io/jac18281828/rust:latest

ARG PROJECT=diehard3
WORKDIR /workspaces/${PROJECT}

USER rust
ENV USER=rust
ENV PATH=/home/${USER}/.cargo/bin:$PATH:/usr/local/go/bin
# source $HOME/.cargo/env

COPY --chown=rust:rust . .

RUN yamlfmt -lint .github/*.yml .github/workflows/*.yml

RUN cargo fmt --check
RUN cargo clippy --all-features --no-deps -- -D warnings
RUN cargo test
RUN RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
RUN python3 -m py_compile generator/bin/find_container_puzzle.py
