FROM gitpod/workspace-full

# Copy project source code
COPY . .
ADD --chown=rust:rust . .

# Print versions information
RUN rustc --version
RUN cargo --version
RUN rustup --version
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.16.12/install)"
RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
RUN avm install latest
RUN anchor --version