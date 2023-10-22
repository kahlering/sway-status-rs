

# sway-status-rs
sway-status-rs shows information about the system in the Sway status bar.
![sway-status-rs](https://github.com/kahlering/sway-status-rs/assets/22997384/1a62fdbc-667a-4398-b8d9-c5a25b4a648e)

# Installation
## Prerequisites
- sway-status-rs is compiled and installed with the Rust package manager Cargo so you will have to install it if it is not installed already.

- sway-status-rs uses alsa-lib to get the audio volume. In Debian it can be installed with:
```
apt install libasound2-dev
```

## Compiling
Next, download the source code from the repository and run the following command in the root dir of the repository:
```
cargo install --path .
```
This should place a executable called sway-status-rs in ~/.cargo/bin

Optionally, you can copy the config.toml to _~/.config/sway-status-rs/config.toml_ and adjust it to your liking. sway-status-rs will look there for the config file. If this file does not exist a default config will be used. 

## Configuring Sway
Now, it is time to configure Sway to use the generated executable. 
Open the sway config file _~/.config/sway/conf_ and replace the status_command with:
```
status_command exec ~/.cargo/bin/sway-status-rs
```
Once you reload the sway config, sway should start displaying the status information.

