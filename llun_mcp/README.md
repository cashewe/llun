# Llun MCP

Llun MCP puts Llun into agentic workflows directly, allowing agents to leverage the users architectural rules during code generation tasks rather than waiting until completion to manually apply the tool.

It is intended to expose 2 tools to the agents:

- 'view_rules': agents can call this tool to view the users selected rules in json format, and make decisions as to how to follow them
- 'review_code_architecture': agents can call this tool to run llun against their generated code

and will be built using the [official rust mcp crate](https://github.com/modelcontextprotocol/rust-sdk/), by following the provided examples.