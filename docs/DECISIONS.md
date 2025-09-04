# Decision Log

I'll put information in here about technical decisions made along the way.

## Initial prototype in python

despite the project being setup specifically to learn rust, the original prototype has been built in python to help put flesh on the bones of the idea. This has allowed me to play around with architecture in a more real way than if i had been simply designing on paper prior to writing.

This python prototype will likely stay in the repo to allow it to be used as a reference for future developments, but is not intended to recieve any ongoing updates aside from to test features. Based on rusts unique architectural differences from python, it is likely to drift away from python in terms of implementation details too, so think of this more as a POC then any kind of useful code.

## openai gpt-4 support only

Initially, supporting just one model will hopefully help to keep focus on the core tool design over collecting support for many models (i have a habit of chasing quick dopamine that would likely lead to 200 models supported and no features otherwise). this decision should be revisited in future, and the code design should be such that it supports multiple model extensability.

## JSON structured rules

Use of JSON to structure the linting rules was done to help ensure that the model recieves the minimum information it needs to make informed decisions. having each rule in a labeled JSON will hopefully make them easier to document and refer back to as well, although time will tell if that will work. The schema of these jsons is open to change, and is largely vibes based at the minuet - in particular the 'example' key felt neccessary but ends up seemingly adding nothing. equally, since these (and infact most) rules are all common idioms, deep descriptions may be redundant, and simply providing the rules short name may well be enough to get the model to know what it needs to do with the rule? worth exploring in future.

This also makes adding custom rules particularly easy for users, as they can simply write json files of their own containing new rules, and point to the directory from their toml.

## defaulting only to .py files

Theres no real reason for this other than limiting token usage, if theres a reason to make this more than just a python tool we can go ahead. Users are able to explicitly provide paths for non python files to the model as in my mind they might find use in being able to pass design documents etc... over so prehaps thats argument enough...

