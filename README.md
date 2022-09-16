# Yet Another Bevy Project

## TODO

- asdf

## Technical Design

### Level Management

If you are in Level A and you want to go to Level B, all you have to do is broadcast the
`core::level_manager::ExitLevel` event!

It takes a `core::level_manager::LevelName` enum, which represents the destination level
to which you want to go.

The destination level will be listening for a `core::level_manager::EnterLevel` event, and
then it will spawn all of the necessary assets to "assemble" the level for you.

Behind the scenes, there is a listener that maps the `ExitLevel` event to the `EnterLevel` event.
The only thing it does before broadcasting the `EnterLevel` event is to delete all of the existing
assets in the world which ahve an `core::level_manager::UnloadOnLevelChange` component label attached.