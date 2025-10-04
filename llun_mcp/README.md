# Llun-MCP - Architectural Context for Agents

## Intro

Llun-MCP extends [Llun](https://pypi.org/project/llun/) into the world of agentic workflows through the [Model Context Protocol](https://modelcontextprotocol.io/docs/getting-started/intro) (MCP). While the Llun CLI tool helps developers enforce architectural principles during coding, Llun-MCP gives agents the same advantage — ensuring they design and generate solutions that align with **your** architectural rules from the very beginning.

Rather than waiting until code is complete to lint and fix, Llun-MCP injects your team’s architectural principles directly into the agent’s reasoning loop. This prevents the kind of ad-hoc, inconsistent output that often plagues LLM-driven code generation, and minimises the need for post agent clean-up.

To achieve this, Llun-MCP exposes a single, simple tool: `get_rules` - which provides the agents with the user selected architectural rules defined by their chosen config tomls. Through encouraging agents to utilise this tool prior to beginning coding tasks, Llun-MCP ensures all agentic workflows begin with a complete understanding of the underlying principles the solution should adhere to. By utilising STDIO, Llun-MCP remains fully portable to all agentic workflows regardless of networking setups, choice of LLM, etc...

All of this makes Llun-MCP perfect for teams that want to:

- Use agents as coding assistants without sacrificing design consistency
- reduce cost of agentic coding activities by ensuring the first results produced are close to the teams agreed standards
- Utilise Llun CLI for architectural assessment
- ensure agent-powered high velocity development doesnt come with a sacrifice in maintainability

## Quick Start

### Installation

to use the server, the fastest way is to pip install it into a local environment:

```
uv pip install llun-mcp
```

or for those not yet ready to migrate to uv:

```
pip install llun-mcp
```

### Basic Usage

To use the tool in CLaude Code, add the following to your config:

```
{
  "mcpServers": {
    "llun_architectural_rules": {
      "command": "uvx",
      "args": [        
        "llun-mcp"
      ]
    }
  }
}
```

## References

- [rmcp](https://docs.rs/rmcp/latest/rmcp/)
- [publishing server guide](https://www.devshorts.in/p/how-to-host-your-mcp-server)