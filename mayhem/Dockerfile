# Build Stage
FROM ubuntu:22.04 as builder

## Dependencies
RUN apt update && apt install -y curl pkg-config cmake build-essential libxcb1-dev libfontconfig-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly && \
    ${HOME}/.cargo/bin/cargo install -f cargo-fuzz

## Add source code to the build stage.
ADD . /src
WORKDIR /src

## Dependencies
RUN apt update && apt install -y libasound2-dev

RUN echo building instrumented harnesses && \
    bash -c "pushd fuzz && ${HOME}/.cargo/bin/cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/fbx_document /fbx_document && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd fuzz && ${HOME}/.cargo/bin/cargo +nightly -Z sparse-registry build --release && popd" && \
    mv fuzz/target/release/fbx_document /fbx_document_no_inst && \
    echo done

# Package Stage
FROM ubuntu:22.04

COPY --from=builder /fbx_document /fbx_document_no_inst /
COPY --from=builder \
        /lib/x86_64-linux-gnu/libxcb.so.1 \
        /lib/x86_64-linux-gnu/libfreetype.so.6 \
        /lib/x86_64-linux-gnu/libfontconfig.so.1 \
        /lib/x86_64-linux-gnu/libasound.so.2 \
        /lib/x86_64-linux-gnu/libXau.so.6 \
        /lib/x86_64-linux-gnu/libXdmcp.so.6 \
        /lib/x86_64-linux-gnu/libpng16.so.16 \
        /lib/x86_64-linux-gnu/libz.so.1 \
        /lib/x86_64-linux-gnu/libbrotlidec.so.1 \
        /lib/x86_64-linux-gnu/libexpat.so.1 \
        /lib/x86_64-linux-gnu/libuuid.so.1 \
        /lib/x86_64-linux-gnu/libbsd.so.0 \
        /lib/x86_64-linux-gnu/libbrotlicommon.so.1 \
        /lib/x86_64-linux-gnu/libmd.so.0 \
        /lib/x86_64-linux-gnu/
