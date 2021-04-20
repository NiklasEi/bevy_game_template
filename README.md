# A Bevy game template

Template for a Game using the awesome [Bevy engine][bevy] featuring out of the box builds for Windows, Linux, macOS and Web (WASM).

_Since Bevy is in heavy development, there regularly are unpublished new features or bug fixes. If you like living on the edge, you can use the branch `bevy_main` of this template to be close to the current state of Bevys main branch_
 
# What does this template give you?
* basic setup with an executable crate on the root level and your game as a Bevy plugin in a library
* small example game (*warning: biased; e.g. split into a lot of plugins and using `bevy_kira_audio` for sound*)
* workflow for GitHub actions creating releases for Windows, Linux, macOS and Web (WASM) ready for distribution
    * push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow

# How to use this template?
 1. Create a repository based on this template
 2. Look for `ToDo` to use your own game name everywhere
 3. [Update the icons as described below](#updating-the-icons)
 4. Start coding :tada:
    * Start the native app: `cargo run --features native`
    * Start the web build: `cargo make serve` (requires `cargo-make`; to install run `cargo install cargo-make`)

You should keep the `credits` directory up to date. The release workflow automatically includes the directory in every build.
 
### Updating the icons
 1. Replace `build/windows/icon.ico` (icon used for windows executable and as favicon for the web-builds)
 2. Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel png icon and run `create_icns.sh` (make sure to run the script inside the `macos` directory) - _Warning: sadly this seems to require a mac..._

# Getting started with Bevy

You should checkout the [bevy website][bevy] for [links to resources][bevy-learn]. I can also recommend the [official Discord server][bevy-discord] as a place to keep up to date with the development and get feedback + help from other Bevy users. 

# Known issues

*  Audio in web-builds might sound bad in some browsers. See [bevy_kira_audio/#9][firefox-sound-issue] for more information.

# License

This project is licensed under [CC0 1.0 Universal](LICENSE) except some content of `assets` and the Bevy icons in the `build` directory (see [Credits](credits/CREDITS.md)). Go crazy and feel free to show me whatever you build with this ([@nikl_me][nikl-twitter]).

[bevy]: https://bevyengine.org/
[bevy-learn]: https://bevyengine.org/learn/
[bevy-discord]: https://discord.gg/bevy
[nikl-twitter]: https://twitter.com/nikl_me
[firefox-sound-issue]: https://github.com/NiklasEi/bevy_kira_audio/issues/9
