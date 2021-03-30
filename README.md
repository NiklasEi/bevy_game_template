# A Bevy game template

Template for a Game using the awesome [Bevy engine][Bevy] featuring (almost) out of the box builds for Windows, Linux and MacOS.

*Fair warning: this template currently uses the master branch of Bevy*
 
# What does this template give you?
* basic setup with an executable crate on the root level and your game as a Bevy plugin in a library
* small example game (*warning: slightly biased; using `bevy_kira_audio` for sound*)
* workflow for GitHub actions creating releases for Windows, Linux and MacOS ready for distribution
    * push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow

# How to use this template?
 * Create a repository based on this template
 * Look for `ToDo` or occurrences of `bevy_game`/`BevyGame` to use your own game name everywhere
 * Replace the icons `build/icons/icon_1024x1024.png` and `build/windows/icon.ico`
    * To get all the mac icons run `build/icons/create_icns.sh` (sadly this seems to require a mac...)
 * Start coding :tada:

# Getting started with Bevy

You should checkout the [bevy website][Bevy] for [links to resources][Bevy-learn]. I can also recommend the [official Discord server][Bevy-discord] as a place to keep up to date with the development and get feedback + help form other Bevy users. 

# Todo

- [ ] Add wasm build

# License

This project is licensed under [CC0 1.0 Universal](LICENSE) except the content of `assets` and the Bevy icons in the `build` directory. Go crazy and feel free to show me whatever you build with this ([@nikl_me][Nikl-twitter]).

[Bevy]: https://bevyengine.org/
[Bevy-learn]: https://bevyengine.org/learn/
[Bevy-discord]: https://discord.gg/bevy
[Nikl-twitter]: https://twitter.com/nikl_me
