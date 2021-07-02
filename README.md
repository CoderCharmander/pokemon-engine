# pokemon-engine

`pokemon-engine` is a modular game engine for Pokémon-like, turn based games. Note that it does
not contain any pre-written moves, monsters or other assets: loading data such as this must be
done by the user.

It is written with the goal to be easy to extend, without hardcoding any moves or special abilities.
This goal is reached with an efficient "effect" system, where effects can react to events
happening in the battle and change data, such as HP and stat stages.

Moves can attach effects to the monsters battling.