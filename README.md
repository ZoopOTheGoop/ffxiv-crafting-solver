FFXIV Crafting
==============

**NOTE: If you're viewing this project and it looks inactive, my current push to simulator completion is on other branches, which makes this project appear unupdated. View the pull requests or the drop down and look at those to see current progress!**

This repository is in-development. It is meant to be a solver for crafting in Final Fantasy XIV. Currently it only consists of a simulator (./ffxiv-crafting), and a bare-bones trait implementation of a derivative of reinforcement learning I call "structured rewards" which extends the concept of numerical rewards to arbitrarily ordered structural types. After verification that the simulator is working, it should be fairly easy to merge these two into a proof-of-concept. Another potential avenue is simple evolutionary algorithms, it will take some testing to decide which works best. 

Worst case, a C API can be made and we can do something like [transformer-based RL](https://arxiv.org/pdf/2106.01345.pdf) via Python FFI, but I'd prefer to avoid that complexity if possible. The transformer paradigm is better than deep actor-critic RL or DQN type stuff for this particular problem, however, since we need to extract an unconditional policy to be executed absent the model, and transformers (broadly) operate as sequence modellers rather than value function chasers, which is what we want.

Note that solving crafting in a policy sense for FFXIV is not particularly difficult. If you were willing to run a bot, you'd probably be better off with just a normal deep RL agent, or even a hand-written script or tree search with some heuristics. The difficulty comes in generating an in-game macro (making this not against the TOS), since every action must unconditionally work regardless of the condition state. There doesn't seem to be a lot of research on AI meant to be executed under heavy constraints (i.e. the policy will be executed by a human in the field without access to the agent's feedback, or must be an unconditional policy), which is where the complexity actually comes in.

Ideally, this will be able to be extended in order to also be able to provide materia and food breakpoints, allowing a search over gearsets to find the cheapest materia you can get that will allow you to HQ a high difficulty recipe reliably.

Contribution welcome, but this project's license currently prohibits forking until it's in a ready state (FFXIV Teamcraft's simulator is likely what you want to use at the moment if you need one of those).
