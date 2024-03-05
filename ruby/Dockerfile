# Stage 1: Build rimage gem
# FROM rust:latest as builder

# WORKDIR /app

# # Install dependencies for building rimage
# RUN apt-get update && apt-get install -y \
#   ruby \
#   ruby-dev \
#   build-essential

# RUN gem install bundler

# RUN apt-get install -y libclang-dev

# # Copy the source code for rimage
# COPY rimage .

# # Install ruby deps
# RUN bundle install

# # Build rimage gem
# RUN bundle exec rake build

# Stage 2: Build server image
FROM ruby:3.2.2

# Add build deps
RUN apt-get update && apt-get install -y libclang-dev build-essential ruby-dev

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Build the library
WORKDIR /app/rimage

COPY rimage .

# Make bundler happy by initializing a git repo
# If the library was it's own git repo it would probably be fine
RUN git init
RUN git add .

RUN bundle install

RUN apt-get install -y ruby-dev

RUN bundle exec rake build
# RUN gem install pkg/rimage-0.1.0.gem

# Setup the server
WORKDIR /app/server

# Copy the server source code
COPY server .

# Build the server image
RUN bundle install

# Set the entrypoint for the server image
CMD ["bundle", "exec", "ruby", "server.rb"]
# CMD ["/usr/bin/bash"]