

# sway-status-rs
sway-status-rs shows information about the system in the Sway status bar.
![sway-status-rs](https://github.com/kahlering/sway-status-rs/assets/22997384/8c72db80-85ee-42e5-be16-f54845604127)

## Installation
### Prerequisites
- sway-status-rs is compiled and installed with the Rust package manager Cargo so you will have to install Cargo if it is not installed already.

- sway-status-rs uses alsa-lib to get the audio volume. In Debian it can be installed with:
```
apt install libasound2-dev
```

### Compiling
Next, download the source code from the repository and run the following command in the root dir of the repository:
```
cargo install --path .
```
This should place a executable called sway-status-rs in ~/.cargo/bin

Optionally, you can copy the example config.toml from the repository to _~/.config/sway-status-rs/config.toml_ and adjust it to your liking. sway-status-rs will look there for the config file. If this file does not exist a default config will be used. 

### Configuring Sway
Now, it is time to configure Sway to use the generated executable. 
Open the sway config file _~/.config/sway/conf_ and replace the status_command with:
```
status_command exec ~/.cargo/bin/sway-status-rs
```
Once you reload the sway config, sway should start displaying the status information.


## Writing your own Module
say-status-rs can be exented by adding a module to display your custom information. This requires the following steps.
- create a struct that implements the StatusModule trait. The trait includes the two functions _get_update()_ and _handle_event()_.
  _get_update()_ is called every refresh and returns the information that will be displayed.
  _handle_event()_ is called if the user clicked on the module in the status bar.
- add your module to the _create_status_module(module_type: &str, module_conf: &toml::Value)_ function in _status_bar/status_module_factory.rs_. This function is used create the necessary modules at the start. It is called once for every module from the config.toml. The module_type is the module name from config.toml and module_conf is a toml::Value that points to the config of that module. The config can be passed to the constructor of your module or be ignored if no config is necessary.
- add a modules entry in the _config.toml_ in _~/.config/sway-status-rs/config.toml_.
  Here is an example for the audio_volume module that is responsible for displaying the audio volume of the default audio device.
  ```
  [modules.audio_volume]
  status_bar_pos = 2
  ```
  The module name, "audio_volume" in this case, will be passed to the create_status_module(module_type: &str, module_conf: &toml::Value) as module_type, so make sure the name matches the implementation.

