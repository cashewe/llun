# Dull

The Dull tool will make use of LLMs to create a 'strategic linter' - a tool which can be used to help guide a unified view of technical strategy within a team, despite 'strategy' being something which cannot be boolean identified via a traditional linter. It'll do this by allowing users to select a number of configurable 'rules' to enforce upon their code such as 'single use principle' etc... and then the tool will raise its concerns to the user in hopefully readable ways.

The target audience would be companies looking to super charge their python use without experienced staff engineers on hand to review every PR etc...

## TODO:

I've broken the roadmap down into three phases:

1. - MVP: this is focused on getting the simplest possible python product working and building as a package. It should act to prove the concept will work
2. - rust refactor: here, i will rewrite the python modules into rust as a means of getting to know the language. for this phase i will allow the use of llm support to answer linguistic questions but will try to avoid leaning on it too hard as i want to actually learn rust. this may therefore take some time and come out pants
3. - beyond MVP: adding additional features and integrations to flesh the product out a bit. it would be nice to get some cool features in during this phase

In the name of getting something out the door, I'd like v1.0.0 to hit pypi within an arbitrary 2 months. this should allow time to convert to rust and get a reasonably stable MVP working in that language, but we may need to reassess if that language becomes harder to figure out than expected

Phase 1 - MVP
-------------
COMPLETE - see src/dull
this code will remain for refference but will no longer recieve active support.

Phase 2 - rust refactor
-----------------------
the results of this process so far are in the 'dull/src' directory. dont ask me why i chose to just swap the folder names and act like thats legible. i dont have an answer other than the cargo initialisation did it for me.

- learn basic rust [X]
- rewrite config manager in rust []
- rewrite openai client in rust []
- rewrite prompt manager in rust []
- rewrite cli in rust [X]
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