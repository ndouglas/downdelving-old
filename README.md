# Downdelving
**Downdelving** is an experimental roguelike written in Rust.  It's built upon the base supplied by [Bracket's Rust Roguelike Tutorial](https://github.com/amethyst/rustrogueliketutorial).  It's a rewrite of [Azymus](https://github.com/ndouglas/azymus), my first experiment in this direction.

You can play the game [in your browser](https://ndouglas.github.io/downdelving/) (assuming your browser supports it).

## Yet Another Roguelike?  Why?  What makes this one different?

The roguelike form is restrictive, but in such a way that it allows and encourages tremendous variation.  It's like a verse-chorus-verse song in that regard.

I have some goals/aims I hope to achieve with this game that may make it somewhat distinctive in the space:

### Artificial Intelligence

**Interesting and original behavior by each creature in the game, reflecting how environmental, historic, cultural, and other pressures have shaped the creature's species, local culture (if any), and the creature as an individual**

I had a powerful experience in the midst of one of the best games I've ever played -- _After the Plague II_, a Hungarian MUD -- where a goblin village had many scripted (but well-scripted) interactions between the goblins -- showing interactions between different classes, genders, ages, etc.  They would speak to each other, they would pick up objects, they would irritate each other, they would fight, and so forth.

It was scripted, of course, and if you killed the goblins they'd respawn a few moments later... but the experience of discovering this village has stuck with me.  And the thought that maybe I could expand on this, and make these interactions more dynamic and more interesting.

I would like to create a simulation in which the player can sit and observe the interactions of a group of goblins, rather than see them merely as blue `g`s to mow down.  True, top-down ASCII maps aren't necessarily the best format for this; so I expect to expand the text ("log") area somewhat, but also improve the UI and mouseover tooltips, etc, to permit deeper insight into each creature and its inner state.

### Visual Beauty and Interest

**An attractive, appealing user interface that stimulates the player's imagination so that they may feel a sense of wonder befitting the world they're exploring**

Roguelike's aren't generally know for their beauty (although [Brogue](https://sites.google.com/site/broguegame/) is lovely and Dwarf Fortress is at least interesting and intricate).  I am not visually artistic and know next to nothing about color theory or anything else that might help me here.  I've never really made any games, designed any attractive websites, or even studied art.  But I would like to make something beautiful.

I know, from playing _Zork_ and _ADoM_ and _Wizardry_ and _Super Mario Bros._ and _Minecraft_ and so forth, that a classically beautiful visual experience isn't necessary to stimulate the player's senses and immerse them in a world.  But I would like to try, and given that I'm trying to create a distinctive roguelike experience, I think the visuals need to be somewhat distinctive as well.  And I think this can be done with ASCII -- I've not really been impressed with most of the tile-based roguelikes I've seen, even the ones with great artwork.  I think it often ends up looking _more_ primitive, rather than less.
