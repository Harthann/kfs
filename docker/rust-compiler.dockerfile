FROM rustlang/rust:nightly-slim

RUN cargo install xargo
RUN rustup component add rust-src

RUN echo '#! /bin/bash\nld -m elf_i386 $@' > /usr/bin/i386-elf-ld

RUN echo '#! /bin/bash\nar rc $@ --target=elf32-i386' > /usr/bin/i386-elf-ar

RUN chmod +x /usr/bin/i386-elf-ar /usr/bin/i386-elf-ld

WORKDIR /root
ENV RUST_TARGET_PATH=/root

ENTRYPOINT ["/bin/bash", "-c"]
