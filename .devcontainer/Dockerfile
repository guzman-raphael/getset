# DevContainer image

FROM rust:1.85-slim
RUN \
    adduser --system --disabled-password --shell /bin/bash --home /home/vscode vscode
RUN \
    # dev setup
    apt update && \
    apt-get install sudo jq git bash-completion graphviz rsync -y && \
    usermod -aG sudo vscode && \
    echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers && \
    echo '. /etc/bash_completion' >> /home/vscode/.bashrc && \
    echo 'export PS1="\[\e[32;1m\]\u\[\e[m\]@\[\e[34;1m\]\H\[\e[m\]:\[\e[33;1m\]\w\[\e[m\]$ "' >> /home/vscode/.bashrc && \
    chown vscode:nogroup /home/vscode/.bashrc && \
    apt-get clean

USER vscode
ENV CARGO_HOME=/home/vscode/.cargo \
    PATH=/home/vscode/.cargo/bin:/home/vscode/.local/bin:$PATH
RUN \
    # rust auto formatting
    rustup component add rustfmt && \
    # rust style linter
    rustup component add clippy && \
    # rust code coverage
    cargo install cargo-llvm-cov && \
    rustup component add llvm-tools-preview && \
    # rust crate structure diagram
    cargo install cargo-modules && \
    # expand rust macros (useful in debugging)
    cargo install cargo-expand
CMD ["bash", "-c", "tail -f /dev/null"]
