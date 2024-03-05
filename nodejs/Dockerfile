# Start from Node.js image
FROM node:latest

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set environment variables for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /app/rimage

COPY rimage .

RUN npm install
RUN npm run build --release

WORKDIR /app/server

COPY server .

RUN npm install

# Run your application
CMD [ "node", "index.js" ]
