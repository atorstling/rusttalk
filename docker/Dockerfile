from debian
RUN apt-get update
RUN apt-get -y install curl
RUN curl https://sh.rustup.rs -sSf > rustup.sh
RUN chmod +x ./rustup.sh
RUN ./rustup.sh -y --default-toolchain nightly-2017-03-13
RUN RUST_BACKTRACE=full ~/.cargo/bin/rustup run nightly-2017-03-13 cargo --version
