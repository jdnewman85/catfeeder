FROM ekidd/rust-musl-builder:nightly as build
COPY --chown=rust:rust ./ ./
RUN touch out
RUN cargo +nightly -Z unstable-options build --out-dir=out
CMD ["bash"]

#FROM alpine
#RUN apk add --no-cache bash git vim file
##COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/debug/catfeeder /out/
#COPY --from=build /home/rust/out/catfeeder /out/
#CMD ["bash"]
