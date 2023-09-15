FROM gitpod/workspace-full

# Copy project source code
COPY . .
ADD --chown=rust:rust . .

# Print versions information
RUN rustc --version
RUN cargo --version
RUN rustup --version
