FROM mcr.microsoft.com/devcontainers/rust:1

USER root
RUN apt-get update -y \
&& apt-get install -y libwebkit2gtk-4.0 libgtk-3-0 libappindicator3-1 fonts-noto-cjk libssl-dev perl cpanminus  webkit2gtk-driver sqlite3 \
&& cpanm JSON IO::Socket::SSL Net::SSLeay LWP::Simple LWP::Protocol::https

USER vscode