#!/bin/bash
set -eu

echo This is postCreate Script!

sudo mkdir -p /workspaces/docbrowser
sudo chown -R 1000:1000 /workspaces
mkdir -p /workspaces/docbrowser/node_modules

sudo chmod -R g+w /usr/local/cargo /usr/local/rustup
sudo chown -R :rustlang /usr/local/cargo
sudo chown -R :rustlang /usr/local/rustup

# cpanm --local-lib=~/perl5 local::lib && eval $(perl -I ~/perl5/lib/perl5/ -Mlocal::lib) \
# cpanm JSON Net::SSLeay LWP::Simple LWP::Protocol::https

# cargo install tauri-cli@1.5.2
pnpm update @tauri-apps/cli @tauri-apps/api --latest
pnpm outdated @tauri-apps/cli

pnpm i