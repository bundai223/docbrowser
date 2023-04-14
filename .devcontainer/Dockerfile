FROM mcr.microsoft.com/devcontainers/rust:1

RUN cargo install tauri-cli@1.2.3
USER root
RUN chmod -R g+w /usr/local/cargo
RUN apt-get update -y && apt-get install -y libwebkit2gtk-4.0 libgtk-3-0 libappindicator3-1 fonts-noto-cjk