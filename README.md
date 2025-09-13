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
- rewrite openai client in rust [X]
- rewrite prompt manager in rust [X]
- rewrite file manager in rust [X]
- rewrite rules object in rust [X]
- rewrite cli in rust [X]
- wrap rust cli in python forwarding logic [X]
- write pyproject support [X]
- implement gitignore support [X]
- tweak the prompt, rule structure etc... until it responds reliably with useful tips [X]
- write a full set of initial design principles that may be of use to people (aim for say 20) [X]
- write new readme for package, including cool emblem []
- write contribution guide []
- paste an actual license []
- write a changelog []
- publish to pypi despite the whole cargo crate being rust based []

Phase 3 - beyond MVP
--------------------
The result of this stage will be a delivery ready tool, but still with bare minimum behaviours. Once this is done, the solution is complete, though we can choose to dip in to any of the remaining tasks to add brand new behaviours.

- write filetype limits (i.e. it cant parse pdf etc... unless i build readers for it, so limit it to plaintext filetypes) []
- define output formats []
- write CICD process to build package, and lint the rust code etc... []
- write unit tests for the relevant behaviours []
- write alternate model source (azure openai is the obvious one, maybe claude too?) []
- write default values into a toml, rather than a quirky text file []
- allow custom written rules []
- allow custom 'context' to be inserted into the prompt such as the tools name and one line description, or the commit / pr description etc... []

## in progress

the code is pretty poor for the filemanager especially - i think the python prototype was super helpful but has lead to me trying to copy it a bit too closely, and pythons lack of ownership means that that leads to bugs in rust. the result of that is alot of slap dash borrowing, error mapping, etc... equally, the filemanager struct has no attributes at all which makes me think it might benefit from a re-architecting. in the python version, i had no distinction between the manager and the fileset whereas here i copied the rules pattern for consistancy. maybe that was a mistake?

I think I'll get MVP up and running this week, and then loop round to better error handling etc... after the fact. id imagine the code with all its loops etc... is slower than it needs to be too which may as well try to keep it fast given thats one of the big selling points of rust.

quick and dirty v1 seems achievable fairly quickly, i simply need to figure out a way to build and deploy the solution, and will need to write some fleshed out docs.
after that itll just be feature additions!
how exciting...

## build guide

1. `source .venv/bin/activate`
2. `uv pip install maturin twine`
3. `maturin develop`
4. `uv run llun --help`

*note* for reasons not yet clear to me, maturin doesnt seem to consistantly install the newest version of the codebase when running maturin develop. you may have some luck by fully wiping the venv and installing llun from wheel? alternately its fairly straight forwards to instead test code changes using `cargo run check ...`.