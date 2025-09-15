# Llun - The Architectural 'Linter'
 
## Intro

Llun ([pronounced '/ɬiːn/'](https://www.howtopronounce.com/welsh/llun), meaning 'picture' (as in **big**)) brings architectural principles directly into your development workflow through a familiar command-line interface. Unlike traditional linters that focus on syntax and style, Llun evaluates your code against configurable architectural rules — ensuring consistency in design patterns, dependency management, and structural decisions across your entire codebase. By leveraging the power of rust, Llun offers a reliable, high performance and type-safe solution to your teams architectural CI/CD needs.

## Why Llun?

Modern development teams face a challenge: how to maintain architectural consistency while leveraging LLM-generated code and supporting developers with varying levels of experience. Traditional code reviews catch surface-level issues but often miss deeper architectural concerns. Raw LLM feedback is inconsistent and context-dependent.
Llun bridges this gap by providing:

- Consistent architectural guidance that goes beyond syntax checking
- Configurable rules tailored to your team's architectural principles
- LLM-powered analysis with the reliability of a traditional linter
- Clear, actionable feedback that helps teams maintain design coherence

Perfect for teams that want to:

- Ensure high-quality commits from developers at all experience levels
- Maintain consistency when incorporating LLM-generated code
- Standardize architectural feedback across code reviews
- Prefer systematic automation over ad-hoc creative decisions

## Quick Start

Follow this guide to get up and running ASAP

### Installation

to use the app, the fastest way is to pip install it into a local environment:

```
uv pip install llun
```

or for those not yet ready to migrate to uv:

```
pip install llun
```

to check installation has worked, run `llun` in the command line to view the help menu for the application.

### Basic Usage

To run Llun in your local directory, use the command

```
llun check .
```
If it is running correctly, you should (eventually) see a json formatted response, explaining to you areas of architectural weakness in the provided code.

**Note** You will need to have set the `OPENAI_API_KEY` variable in your environment to a valid openai api key in order to get valid output from the service. New api keys can be generated at [this address](https://platform.openai.com/api-keys). Users should be cognisant of the associated costs to run their chosen AI model.

## Configuring Llun

Llun makes use of a heirarchical configuration under the following rules:

1. Llun has a sensible set of default values, which can be directly observed in `src/data/default.toml`
2. Use of the `tool.llun` tag in `pyproject.toml` will overwrite any defaults
3. Use of a `llun.toml` will overwrite any prior configurations 
4. any CLI arguments override everything prior

At the moment, Llun does not support any nested configuration. If you require this feature (for instance for a monorepo), feel free to develop it and submit a PR.

### API Guide

the following table describes the various methods available to the `llun check` command. It is kept up to date with the currently deployed package.

| Argument | Description | Valid Values | Default |
|----------|-------------|--------------|---------|
| `--path`   | The directory or file to run llun against | Any path from root i.e. './XXX' or '.' | None |
| `--exclude` | A path to be excluded from the targeted directory described by `--path` | Any path from root i.e. './XXX' or '.' | None | 
| `--select` | A (valid) Llun rule code to apply during the check | Any rule code i.e. 'LLUN01' | ['LLUN01', 'LLUN02', 'LLUN03', 'LLUN04', 'LLUN05'] (the SOLID principles) |
| `--extend-select` | Extend the rules selected in a lower level of configuration | Any rule code i.e. 'LLUN01' | None |
| `--ignore` | A rule selected at any point prior to be ignored for the current run | Any rule code i.e. 'LLUN01' | None |
| `--model` | An openAI model to use to run the check on | Any OpenAI model | "gpt4-o" |
| `--no-respect-gitignore` | Including this flag will disable the behaviour which automatically `--exclude`s any file in the gitignore (not recommended in case you leak secrets etc...) | False |
| `--output-format` | The format that llun should use for its trace | "json", "azure" | "json" |
| `--provider` | The LLM provider to run the check against | "openaipublic" | "openaipublic" |

This table will be updated as new methods or valid values are encorporated.

### Rule Guide

The following describes each rule that can be set by Llun. full descriptions can be found in the rule files (`/src/data/rules`)

#### LLUN01: Single Responsibility Principle
Each class, function, or module should have one clear responsibility and one reason to change.

#### LLUN02: Open/Closed Principle
Software entities should be open for extension but closed for modification.
  
#### LLUN03: Liskov Substitution Principle
Objects of a superclass should be replaceable with objects of its subclasses without affecting the correctness of the program.
  
#### LLUN04: Interface Segregation Principle
Clients should not be forced to depend on interfaces they do not use.
  
#### LLUN05: Dependency Inversion Principle
High-level modules should not depend on low-level modules. Both should depend on abstractions.

#### LLUN06: Favour Composition Over Inheritance
Build functionality by combining objects that contain other objects, rather than inheriting behavior from parent classes.
  
#### LLUN07: YAGNI (You Aren't Gonna Need It)
Don't implement functionality until it's actually required. Build only what you need right now, not what you think you might need later.

#### LLUN08: KISS (Keep It Simple, Stupid)
Choose the simplest solution that solves the problem effectively. Avoid unnecessary complexity in design and implementation.
  
#### LLUN09: Keep Functions Pure
Functions should always return the same output for the same input and have no side effects (don't modify external state or perform I/O operations).
  
#### LLUN10: Use Ubiquitous Language
Use the same terminology and concepts throughout the codebase, documentation, and conversations that domain experts use in the real business context.
  
#### LLUN11: Keep Context Bounded
Define clear boundaries where specific domain models and business rules apply. Different parts of the system can have different interpretations of the same concept.
  
#### LLUN12: Convention Over Configuration
Provide sensible defaults and follow established patterns so developers can be productive without extensive setup or decision-making about common scenarios.
  
#### LLUN13: Strategy Pattern
Define a family of algorithms or behaviors, encapsulate each one, and make them interchangeable at runtime based on context or configuration.
  
#### LLUN14: Dependency Injection
Provide dependencies to a class from the outside rather than creating them internally. Dependencies should be injected through constructors, methods, or properties.
  
#### LLUN15: Facade Pattern
Provide a simplified interface to a complex subsystem by creating a single entry point that coordinates multiple underlying components.
  
#### LLUN16: Repository Pattern
Encapsulate data access logic behind an interface that mimics a collection of domain objects, separating business logic from data persistence concerns.

#### LLUN17: Idempotency
Operations should produce the same result when called multiple times with the same parameters. Repeated calls should not cause additional side effects or change the system state further.

#### LLUN18: Minimize Stateful Objects
Prefer stateless objects and immutable data structures. Keep mutable state localized and explicit, rather than spreading it throughout your object hierarchy.

#### LLUN19: Immutable Objects
Create objects whose state cannot be modified after construction. When changes are needed, return new instances rather than modifying existing ones.

#### LLUN20: Test behaviour, not implementation
Write tests that verify the system's externally visible behavior rather than its internal implementation details.

## Contributing

We are glad to take contributions via github issues or pull requests into the main branch. Please ensure all code is tested and documented before opening a pull request in order to aid the process along. to test locally, you'll want to follow the build guide below:

### build guide

run the following commands to build the application locally

1. `source .venv/bin/activate`
2. `uv pip install maturin twine`
3. `maturin develop`
4. `uv run llun --help`

note that to rebuild after a change, you'll want to make sure you purge the current app version entirely from your environment.