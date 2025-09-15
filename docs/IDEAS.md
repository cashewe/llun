# Ideas

In this file I'll write future ideas in a bit more depth so i can figure my way through them. The focus should mostly be on what I can learn from since simply developing the tool feels of limited value (lets be real, the tool is generally of limited value lol)

## Implementations

By default, the solution is a cmd tool. this gives easy access, and lets us mimic ruff's implementation to some extent (admittedly i havent reffered back to this in a while - i should check back in). That said there is opportunity to offer alternate means of running the tool that may add value to the users, and some of which could offer unique development challenges to me too.

### MCP Server

Local coding agents are free to run amock, though they can quality assure their own work, a definition of quality that can be consistent across a stack doesnt currently exist i.e.

- precommit rule
- code review
- code generation
- solution design
- etc...

can all have a subjective view of 'good' architecture. If we could offer an MCP server (have to be a local host as no way i can remote host a public solution lol) then we could use the code (mostly the prompts) to create a 'judge' agent behaviour which assess agentically generated code against spec as its designed and can demmand refactors if its not good enough. This will give me the opportunity to develop a complete localhost mcp solution, which i think should be fun and novel, and expose me a bit more to agentic design via the judge pattern. I'm not certain how to develop this in rust - so prehaps I would start with a python prototype again? This one feels the highest value to me.

### github action / devops task

The idea here would be to offer a pull request behaviour which runs the solution against diffed files (or maybe just the raw diff itself?) and then leaves some comments on the triggering pull request based on user supplied rules. This can help to give a cohesive view of architecture both locally via cmd / mcp and remotely via the review. this could be put up on the marketplace for open access to other teams, furthering the open source scope of the tool.
To me, im not sure this offers great learning opportunity - though it may be worth trying it out none the less just in case. the github action option feels more useful to the world at large, though sadly i am at this point more familiar with ADO...

### precommit hook

Zero learning opportunity whatsoever, but pre-commit is pretty popular and its an easy synergy.

## Other Output Types

At the minute, the sole output is json trace. this is *fine* and works well as a building block for other tools, but it doesnt feel as user friendly as it could be. it would be good to consider what alternatives might be possible...

### Junit trace

encorporate junit outputs from the tool, allowing users to log runs. this might not be great since junit is a binary pass / fail outcome which this tool probably wont reliably produce. That said, auditability is important so some kind of standardised log format for runs should be considered, and junit seems the most common at least in pythons ecosystem

### actual stack trace

this might be pants? but it might be usefu to produce a heap that shows the line the issue was detected, and explains the best possible solution.

### pytest esque summary 

if this tool may be run in CICD, having a readable summary output would be useful for users.

### ADO / github (?) output

If this tool is run in CICD, having a formatted summary that those pipeline tools can use would be useful. for ADO, there is the #vsooutput[] syntax, but im not sure if github offers the same / similar

## Further CMD commands

In the name of YAGNI, the product will release with the bare minimum collection of commands. There are bound to be further QoL commands that could be added to make the tool easier to use. for instance if we support custom rules, then allowing users to store them in a specific folder would be good. could llun generate the structure for them with a command? could it generate the rules for them via LLM just because?

### some kind of extension filter

This would let you run on a given directory, but maybe exclusively use or maybe ignore any files of given extension types. 
something not considered in the current design is the case of files which are not code files. We probably still want to be able to parse anything that can be read as plain text to be honest - as this will allow users to pass design documents etc... to the tool and get feedback prior to building their solutions if they so wish so supporting more than .py is a value add. That said, we cant support random extensions like .pptx etc..., so we need some solution to filter files in a dir out - and we may as well allow the users to configure that to some extent.
I am unclear on the best implementation here tbh...

### add a #noqa equivalent

obvious

### add an option to have certain rules only run on certain files

obvious

### have rule families and allow users to select whole families at once

obvious

## Support other model calls

For the MVP release, we'd be looking at just offering openAI key access and using a specific model. it'd be good to allow the user to not only pick their model of choice, but also to use more than just openAI. If possible, an option to use your own LLM might be cool, but that may be more for the MCP implementation than it is for the core package. either way anthropic support feels like a neccessity at the bare minimum

## support a --fix flag

this would extend the behaviour to take the output text and actually implement the suggested fixes. This again would likely need to be api call based - potnetially you'd do it async, with a call for each file needing editing and include all the flags for that file in the prompt?
This one feels like it'd teach me alot about rust async, so i think id like to prioritise it too.