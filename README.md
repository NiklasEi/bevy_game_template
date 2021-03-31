# A Bevy game template

Template for a Game using the awesome [Bevy engine][Bevy] featuring (almost) out of the box builds for Windows, Linux and MacOS.

*This template currently uses the master branch of Bevy*
 
# What does this template give you?
* basic setup with an executable crate on the root level and your game as a Bevy plugin in a library
* small example game (*warning: biased; e.g. split into a lot of plugins and using `bevy_kira_audio` for sound*)
* workflow for GitHub actions creating releases for Windows, Linux and MacOS ready for distribution
    * push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow

# How to use this template?
 1. Create a repository based on this template
 2. Look for `ToDo` to use your own game name everywhere
 3. [Update the icons as described below](#updating-the-icons)
 4. Start coding :tada:
 
### Updating the icons
 1. Replace `build/windows/icon.ico` (icon used for windows executable)
 2. Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel png icon and run `create_icns.sh` (make sure to run the script inside the `macos` directory) - _Warning: sadly this seems to require a mac..._ 

# Getting started with Bevy

You should checkout the [bevy website][Bevy] for [links to resources][Bevy-learn]. I can also recommend the [official Discord server][Bevy-discord] as a place to keep up to date with the development and get feedback + help from other Bevy users. 

# Todo

- [ ] Add wasm build

# License

This project is licensed under [CC0 1.0 Universal](LICENSE) except the content of `assets` and the Bevy icons in the `build` directory. Go crazy and feel free to show me whatever you build with this ([@nikl_me][Nikl-twitter]).

[Bevy]: https://bevyengine.org/
[Bevy-learn]: https://bevyengine.org/learn/
[Bevy-discord]: https://discord.gg/bevy
[Nikl-twitter]: https://twitter.com/nikl_me
