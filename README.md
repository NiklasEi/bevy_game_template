# A Bevy game template

Template for a Game using the awesome [Bevy engine][bevy] featuring out of the box builds for Windows, Linux, macOS, and Web (Wasm).

_Since Bevy is in heavy development, there regularly are unpublished new features or bug fixes. If you like living on the edge, you can use the branch `bevy_main` of this template to be close to the current state of Bevy's main branch_
 
# What does this template give you?
* small example ["game"](https://niklasei.github.io/bevy_game_template/) (*warning: biased; e.g., split into a lot of plugins and using `bevy_kira_audio` for sound*)
* easy setup for running the web build using [trunk] (`trunk serve`) 
* run the native version with `cargo run`
* workflow for GitHub actions creating releases for Windows, Linux, macOS, and Web (Wasm) ready for distribution
    * push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow
    * WARNING: if you work in a private repository, please be aware that macOS and Windows runners cost more build minutes. You might want to consider running the workflow less often or removing some builds from it. **For public repositories the builds are free!**

# How to use this template?
 1. Click "Use this template" on the repository's page
 2. Look for `ToDo` to use your own game name everywhere
 3. [Update the icons as described below](#updating-the-icons)
 4. Start coding :tada:
    * Start the native app: `cargo run`
    * Start the web build: `trunk serve`
       * requires [trunk]: `cargo install --locked trunk`
       * requires `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
       * this will serve your app on `8080` and automatically rebuild + reload it after code changes

You should keep the `credits` directory up to date. The release workflow automatically includes the directory in every build.

### Updating the icons
 1. Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel png icon and run `create_icns.sh` (make sure to run the script inside the `build/macos` directory) - _Note: this requires a mac_
 2. Replace `build/windows/icon.ico` (used for windows executable and as favicon for the web-builds)
    * You can create an `.ico` file for windows by following these steps:
       1. Open `macos/AppIcon.iconset/icon_256x256.png` in [Gimp](https://www.gimp.org/downloads/)
       2. Select the `File > Export As` menu item.
       3. Change the file extension to `.ico` (or click `Select File Type (By Extension)` and select `Microsoft Windows Icon`)
       4. Save as `build/windows/icon.ico`

### Deploy web build to GitHub pages
 1. Trigger the `deploy-github-page` workflow
 2. Activate [GitHub pages](https://pages.github.com/) for your repository
     1. Source from the `gh-pages` branch (created by the just executed action)
 3. After a few minutes your game is live at `http://username.github.io/repository`

To deploy newer versions, just run the `deploy-github-page` workflow again.

Note that this does a `cargo build` and thus does not work with local dependencies. Consider pushing your "custom Bevy fork" to GitHub and using it as a git dependency.

# Getting started with Bevy

You should check out the Bevy website for [links to resources][bevy-learn] and the [Bevy Cheat Book] for a bunch of helpful documentation and examples. I can also recommend the [official Bevy Discord server][bevy-discord] for keeping up to date with the development and getting help from other Bevy users.

# Known issues

Audio in web-builds can have issues in some browsers. This seems to be a general performance issue and not due to the audio itself (see [bevy_kira_audio/#9][firefox-sound-issue]).

# License

This project is licensed under [CC0 1.0 Universal](LICENSE) except some content of `assets` and the Bevy icons in the `build` directory (see [Credits](credits/CREDITS.md)). Go crazy and feel free to show me whatever you build with this ([@nikl_me][nikl-twitter] / [@nikl_me@mastodon.online][nikl-mastodon] ).

[bevy]: https://bevyengine.org/
[bevy-learn]: https://bevyengine.org/learn/
[bevy-discord]: https://discord.gg/bevy
[nikl-twitter]: https://twitter.com/nikl_me
[nikl-mastodon]: https://mastodon.online/@nikl_me
[firefox-sound-issue]: https://github.com/NiklasEi/bevy_kira_audio/issues/9
[Bevy Cheat Book]: https://bevy-cheatbook.github.io/introduction.html
[`wasm-server-runner`]: https://github.com/jakobhellermann/wasm-server-runner
[trunk]: https://trunkrs.dev/
