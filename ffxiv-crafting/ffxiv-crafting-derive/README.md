# FFXIV Crafting Proc Derive Macros

Implements proc macros for the crafting simulator. Making these general is a non-goal and only need to work
in the crate itself. Most of the macros are meant to make actions much easier to generate without having to
manually implement traits in most cases.
The line between when to put a generation option in here or not is fairly arbitrary, and largely just comes down
to how often you'll do the same thing without it and how easy it is to add an extension.
Arguably deriving most of the crafting buffs could be part of the buff derive, but they're not because they're
one line and just annoying enough to derive that it's easier to just manually implement buff actions except for
very boilerplatey cases like "touch" actions affecting IQ.