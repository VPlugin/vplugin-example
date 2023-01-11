# About
This application is an example of how to use VPlugin in your application. It's an OpenGL program that creates a context and leaves the rest to plugins, making it easy to understand how to work with it.
Feel free to study its source code, use it as a template or do anything else with it.

<h5><i>Tip: When pressing <kbd>Enter</kbd> the first plugin available will be unloaded until all plugins have been removed.</i></h5>

# Requirements
- OpenGL: >= 3.0 (Modern OpenGL)
- Rust and Cargo (Obviously)

# Installation
You can either clone this repository:
```sh
$ git clone https://github.com/VPlugin/vplugin-example.git && cd vplugin-example/
$ cargo install --path .
```
Or install directly from [crates.io](https://crates.io):
```sh
$ cargo install vplugin-example
```
The first one is the latest possible version you can get, use it for the latest features.

# Plugin Support
The plugins should be conforming to the [VPlugin Plugin Format](https://github.com/VPlugin/VPlugin/blob/master/spec/PluginFormat.md). The entry point should be named `vplugin_example_init`.
To learn about recognizable hooks, see [docs/plugins.rs](docs/plugins.rs)

# License
This application is released under the MIT license. For more information, see [LICENSE](LICENSE).
Plugins may be of any license, there are no restrictions.