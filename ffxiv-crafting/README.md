# FFXIV Crafting Simulator

A simulator definition for FFXIV's crafting. This is largely based off the observations in the [How to Craft Like a Machine][HTCLAM] document, as well as manual observations and digging around in the game files to fill in gaps.

This is a relatively low-level simulator so you're left to manually specify things like a recipe's internal rlvl yourself. Similarly it doesn't calculate things such as the max durability, progress, and quality off a recipe's internal factors (it has no awareness of this) so it's up to you to calculate these when constructing the problem. 

Relatively fast simulation is its main goal, while allowing for flexibility in adding new actions and crafting modes as needed.

Currently it contains a general implementation that can swap between HQ/NQ (normal) crafting and both types of expert crafts. It contains definitions for all actions up through Shadowbringers. The recipe level maps are taken from an abridged table in the [spreadsheet][HTCLAMS] linked in [HTCLAM], except for the `RLVL_MOD` value which is taken from the game files, so there may be some inconsistencies for lower level items (ie mentions there's some weirdness for level 51 recipes for instance). However, this shouldn't be too big of a deal.

Most of the action defintions are done via proc macros to avoid too much trait boilerplate. If you read over the action definitions you should get a general feel for how things are specified fairly quickly.

[HTCLAM]: https://docs.google.com/document/d/1Da48dDVPB7N4ignxGeo0UeJ_6R0kQRqzLUH-TkpSQRc/edit#
[HTCLAMS]: https://docs.google.com/spreadsheets/d/1n8iteSp1Aa4X2_zXxo7j3soxsmik4K1mG6UZiBPBoNU/edit