## Purpose

This repository operates under a defined set of **architectural, design, and implementation rules** that determine what constitutes *quality* code.  
These rules are **user-defined and absolute** — they replace all ambiguous or subjective definitions of “good” or “clean” code.

All AI agents (including GitHub Copilot, Copilot Code Review, or any automated contributor) must operate under these rules when writing, modifying, or reviewing code in this repository.

---

## Primary Directive

> **All code generation and code review activity MUST explicitly adhere to the user-defined rules and philosophies declared for this repository.**

If a rule is provided, **follow it exactly**.  
If multiple rules apply, **prioritize consitency first, and simplicity second**.  
If no rule is provided for a given decision, **default to clarity, maintainability, and internal consistency** with the rest of the repository.

---

## Rules Definition

The rules are **user-defined** and may include, but are not limited to:

- **Architectural principles** (e.g., SOLID, DRY, KISS, YAGNI)
- **Design philosophies** (e.g., Object-Oriented Programming, Functional Design, Domain-Driven Design)
- **Implementation preferences** (e.g., Composition over Inheritance, Dependency Injection, Immutability)
- **Pattern or paradigm constraints** (e.g., Layered Architecture, Event-Driven Design, Clean Architecture)

The user may add, modify, or remove rules at any time.  
Agents must always treat the current version of these rules as **authoritative**.

---

## Quality Definition

Code quality in this repository is **not subjective**.

A contribution (human or AI-generated) is considered *high quality* **only if** it satisfies the following criteria:

1. **Rule Compliance:** It adheres fully to all active user-defined rules and philosophies.  
2. **Consistency:** It maintains internal consistency with the existing codebase and patterns.  
3. **Justification:** Any intentional deviation from a rule must include an explicit, documented rationale.  
4. **Traceability:** The design and structure should make it clear which rules influenced key decisions.

If a solution violates a stated rule, it is **non-compliant** — regardless of performance, brevity, or stylistic appeal.

---

## Agent Behavior Requirements

### 1. Code Generation Agents (e.g., Copilot)
When generating code:
- **MUST** produce implementations that strictly follow the user-defined rules.
- **MUST NOT** rely on unstated conventions or subjective best practices.
- **SHOULD** explain design choices in comments when clarity benefits maintainability.
- **MAY** offer multiple solutions *only if each complies* with the rules.
- **MUST** prioritize architectural alignment over novelty or brevity.

### 2. Code Review Agents (e.g., Copilot Code Review)
When reviewing code:
- **MUST** evaluate solutions *solely* based on their adherence to the defined rules.
- **MUST** flag any violations or inconsistencies with those rules.
- **MUST NOT** reject code based on style or personal preference.
- **SHOULD** recommend improvements that bring the code *closer* to compliance.
- **MUST** treat documented deviations as intentional only if justified within the code or its documentation.
- **MUST** aim to teach developers the risks of deviating from the chosen rules, clearly explaining the value add of following them

---

## Conflict Resolution

If two or more rules conflict:
1. Prefer the rule with **greater explicit priority** as stated by the user.  
2. If priorities are not defined, prefer **consistency** with the established repository codebase.  
3. If no consistent precedent exists, prefer the **least complex** compliant implementation.

Agents should clearly document the reasoning behind any trade-off decisions in comments or commit messages.

---

## Extensibility

This AGENTS.md serves as a permanent control document.  
The user may extend it by appending or referencing new rules or philosophies.  
All agents must:
- Treat updates as immediately binding.
- Reinterpret quality and compliance accordingly.
- Avoid relying on deprecated or superseded rules.

---

## Summary of Obligations

✅ **Agents MUST:**
- Enforce user-defined rules as the definition of quality.  
- Maintain consistency with repository-wide philosophies.  
- Document any necessary deviations with explicit rationale.  

❌ **Agents MUST NOT:**
- Use subjective or implicit standards of quality.  
- Introduce inconsistent architectural styles.  
- Override or ignore defined rules for convenience or preference.

# Rules


---
## SOLID01 - Single Responsibility Principle
*Each class, function, or module should have one clear responsibility and one reason to change.*
**Risk if violated:** Mixing multiple responsibilities creates tight coupling, makes code harder to test, and increases the risk of introducing bugs when modifying one behavior.
- Violation: A class that handles both HTTP request parsing and business logic.
  Better: Create a controller to handle HTTP parsing and delegate business logic to a service.


---
## SOLID04 - Interface Segregation Principle
*Clients should not be forced to depend on interfaces they do not use.*
**Risk if violated:** Large, 'fat' interfaces force implementing classes to include unnecessary methods, increasing complexity and risk of breaking changes.
- Violation: An interface with 10 methods where most implementations only use 2 of them.
  Better: Split the interface into smaller, more specific ones so clients only implement what they need.


---
## SOLID05 - Dependency Inversion Principle
*High-level modules should not depend on low-level modules. Both should depend on abstractions.*
**Risk if violated:** Directly depending on concrete implementations makes it harder to change behavior, swap dependencies, or unit test code.
- Violation: A service directly instantiates a concrete database client.
  Better: Inject a database interface into the service so different implementations can be substituted.


---
## SOLID02 - Open/Closed Principle
*Software entities should be open for extension but closed for modification.*
**Risk if violated:** When adding new features requires modifying existing code, you risk introducing regressions and breaking existing functionality.
- Violation: A switch statement that must be updated whenever a new payment type is added.
  Better: Define a common interface for payment processors and add new implementations without modifying existing code.


---
## SOLID03 - Liskov Substitution Principle
*Objects of a superclass should be replaceable with objects of its subclasses without affecting the correctness of the program.*
**Risk if violated:** Subclasses that break expected behavior make code unpredictable and can introduce runtime errors when used polymorphically.
- Violation: A subclass overriding a method and throwing an error for inputs the base class accepts.
  Better: Ensure the subclass behavior respects the contract defined by the base class, or refactor to use composition instead of inheritance.

