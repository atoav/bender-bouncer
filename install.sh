echo 'Starting to build bender-bouncer\n'
cargo build --release
echo 'Build finished.'
echo 'Attempting to move binaries to /usr/bin:'
sudo cp ./target/release/bender-bouncer-cli /usr/bin/bender-bouncer-cli
sudo chmod +x /usr/bin/bender-bouncer-cli
echo 'Installed bender-bouncer-cli to /usr/bin'