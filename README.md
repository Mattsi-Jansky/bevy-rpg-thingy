# RPG Bevy Thingy

This is a project used for exploring Bevy. It might become the basis of a tutorial or workshop at some point.

## Concepts Demonstrated

* App
  * `main.rs`
* Systems
  * `systems` folder
* Queries
  * `resolve_player_commands`
* Resources
  * `assets` folder, initiated in `init_` functions and queried in `main::setup`
* Events
  * Command event created in `cursor.rs`, acted on in `resolve_player_command.rs`
