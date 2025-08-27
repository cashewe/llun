# Dull

Dull is welsh mate its racist to pronounce it as dull and call me dumb for my boring tool.

The tool will make use of LLMs to create a 'strategic linter' - a tool which can be used to help guide a unified view of technical strategy within a team, despite 'strategy' being something which cannot be boolean identified via a traditional linter.

The target audience would be companies looking to super charge their python use without experienced staff engineers on hand to review every PR etc...

## TODO:

I've broken the roadmap down into three phases:

1. - MVP: this is focused on getting the simplest possible python product working and building as a package. It should act to prove the concept will work
2. - rust refactor: here, i will rewrite the python modules into rust as a means of getting to know the language. for this phase i will allow the use of llm support to answer linguistic questions but will try to avoid leaning on it too hard as i want to actually learn rust. this may therefore take some time and come out pants
3. -beyond MVP: adding additional features and integrations to flesh the product out a bit. it would be nice to get some cool features in during this phase

my aim is to put a full stop on the work and formally release v1.0.0 within a month as not like this is a super important tool or nothing, so if there are boxes left unchecked at that time, they may well remain forever untouched.

Phase 1 - MVP
-------------
- create a default behaviour [X]
- create the option to use toml files to overwrite the defaul behaviour [X]
- refactor the prompts to use the txt files [X]
- create the option to use cmd commands to overwrite the toml behaviour [X]
- get access back to my github so i can test the prototype, and fix any glaring bugs [X]
- trim all unneccessary code away until all thats left is the bare minimum [X]
- ensure the suggested design scales well for future feature extension, potentially could use the tool itself for this just for fun tbh [X]

Phase 2 - rust refactor
-----------------------
- rewrite config manager in rust []
- rewrite openai client in rust []
- rewrite prompt manager in rust []
- rewrite cli in rust []
- wrap rust cli in python forwarding logic []
- publish to pypi despite the whole cargo crate being rust based []

Phase 3 - beyond MVP
--------------------
- support nested pyprojects []
- support dull.toml []
- unify usage syntax with ruff tbh []
- support model configurability []
- support more than just openai models []
- design and create a more useful output than just json []
- support junit []
- write vscode extension []
- write azure pipelines extension []
- write github actions extension []
- improve prompt to ensure model outputs are as expected []