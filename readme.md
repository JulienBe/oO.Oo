# Goal

In one word: simplicity

The goal is to get my first bevy project in a "published" state.

### Gameplay simplicity

A very basic game where the goal would be, on a 2D plane, to get a square into a specific plane at regular intervals.

### Graphics simplicity

Just the basic, a square of color for the player, a square of color for the target

Ideally, that would be swapped easily (yeah, particles, ascii art, I see you coming)

# Level two

Keep the simple design but move to something more dynamic. Giving the impression that the player is skydiving or something (Maybe it's a virus getting into a computer imaginary world ?).

That would imply:
- Fancier graphics
- A camera that follows the player

# Plans

- commit 1: Display a square which is the player
- commit 2: Move that square
- commit 3: Prevent that square from escaping the screen
- commit 4: Spawn a target
- commit 5: Despawn a target after a certain time
- commit 6: Detect collision between the player and the target
- commit 7: Despawn the target if the player collides with it
- commit 8: Display a counter of collected targets
- commit 9: Lose if you do not reach the target in time

# Bumps

### Bevy's [example](https://github.com/bevyengine/bevy/blob/latest/examples/2d/move_sprite.rs) seems to be bundling the sprite into the entity.

Which is something I don't really like, I don't want to deal with that coupling.

I'd rather have the system figure out how it wants to render the entity.

# Compile time tips

### Enable Bevy's Dynamic Linking Feature

This is the most impactful compilation time decrease! 

If bevy is a dependency you can compile the binary with the "dynamic" feature flag (enables dynamic linking).

`cargo run --features bevy/dynamic`

[more here](https://bevy-cheatbook.github.io/pitfalls/performance.html)