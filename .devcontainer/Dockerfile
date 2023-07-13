FROM mcr.microsoft.com/devcontainers/rust:1

RUN cargo install tauri-cli@1.4.0
USER root
RUN chmod -R g+w /usr/local/cargo
RUN apt-get update -y
RUN apt-get install -y libwebkit2gtk-4.0 libgtk-3-0 libappindicator3-1 fonts-noto-cjk libssl-dev perl cpanminus  webkit2gtk-driver sqlite3
RUN cpanm JSON IO::Socket::SSL Net::SSLeay LWP::Simple LWP::Protocol::https
RUN mkdir -p /workspaces/docbrowser && chown -R 1000:1000 /workspaces

USER vscode
RUN mkdir -p /workspaces/docbrowser/node_modules
# RUN : "install perl modules" \
# && cpanm --local-lib=~/perl5 local::lib && eval $(perl -I ~/perl5/lib/perl5/ -Mlocal::lib) \
# && cpanm JSON Net::SSLeay LWP::Simple LWP::Protocol::https
