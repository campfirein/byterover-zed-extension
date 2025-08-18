# ByteRover MCP Server

This extension provides a Model Context Server for ByteRover, for use with the Zed AI assistant.

ByteRover is a self-improving, shared memory layer that remembers your AI agent's coding experiences—enabling smarter, faster development and seamless collaboration across your entire dev team. [Learn more about ByteRover](https://www.byterover.dev/?source=zed).

## Configuration

To use the extension:

1. Visit [ByteRover](https://www.byterover.dev/?source=zed) to create a memory workspace
2. Follow the Space start guide to get your configuration values
3. Configure the context server by setting the `auth_header` in your Zed `<project-root>/.zed/settings.json`

```json
{
  "context_servers": {
    "mcp-server-byterover": {
      "source": "extension",
      "settings": {
        "auth_header": "YOUR_SPACE_AUTH_HEADER"
      }
    }
  }
}
```

## Memory Tools

The ByteRover MCP server provides two core memory management tools:

- **`byterover-retrieve-knowledge`**: Search your curated knowledge base of past coding solutions and insights to find relevant help for your current task
- **`byterover-store-knowledge`**: Capture key coding insights from human-agent interactions to help solve similar tasks in the future

## Rule System for Memory Management

The ByteRover extension uses a flexible rule system that allows you to customize how the AI agent interacts with your memory workspace. Rules define **when** and **how** the agent should use memory operations during development workflows.

### Basic Configuration


```text
# Important: Automated memory workflow
always use byterover-retrieve-knowledge tool to get the related context before any tasks
always use byterover-store-knowledge to store all the critical informations after successful tasks
```

This configuration ensures the agent:
1. **Searches existing memories** before starting any development task to leverage past solutions
2. **Stores new insights** after successfully completing tasks to build your knowledge base

### Customizing Memory Behavior

#### When to Use Memory

You can configure different triggers for memory operations:

```text
# Search memory before specific tasks
always use byterover-retrieve-knowledge before implementing new features
always use byterover-retrieve-knowledge when debugging complex issues
always use byterover-retrieve-knowledge for architecture decisions

# Store memories based on task completion
always use byterover-store-knowledge after successful bug fixes
always use byterover-store-knowledge when completing code reviews
always use byterover-store-knowledge for performance optimizations
```

#### How to Use Memory

Create a `<project-root>/.rules` file and add your custom rules to control how the agent uses memory:

```text
# Context-specific memory usage
use byterover-retrieve-knowledge for similar technology stacks only
use byterover-store-knowledge to store reusable patterns not full implementations
focus byterover-store-knowledge on design decisions and lessons learned

# Memory format guidelines
store coding patterns and architectures in byterover-store-knowledge
capture algorithm explanations and trade-offs in byterover-store-knowledge
include dependency recommendations and gotchas in byterover-store-knowledge
```

#### Advanced Memory Workflows

```text
# Conditional memory usage
use byterover-retrieve-knowledge when working with unfamiliar libraries
use byterover-retrieve-knowledge before refactoring legacy code

# Team collaboration
use byterover-retrieve-knowledge to maintain coding standards across team
store byterover-store-knowledge with clear problem-solution mapping
```

### Explicit Memory Operations

Beyond automated rules, you can explicitly request memory operations:

- **"Search our memories for React hook patterns"** - Manually trigger memory search
- **"Remember this API integration pattern for future use"** - Manually store knowledge
- **"What do we know about handling database migrations?"** - Query specific domain knowledge

### Best Practices for Developers

1. **Start Simple**: Use the default rules initially, then customize based on your workflow
2. **Focus on Patterns**: Store reusable coding patterns, not complete implementations
3. **Context Matters**: Include relevant tech stack and project context in memory rules
4. **Iterate Rules**: Refine your rules based on how effectively they improve your development speed

### Rule Placement

Add your custom rules to your workspace configuration in one of these locations:
- `.zed/settings.json` (project-specific)
- Global Zed settings (applies to all projects)

## Learn More

For more information about ByteRover's features, pricing, and capabilities, visit the [ByteRover homepage](https://www.byterover.dev/?source=zed).

Explore how ByteRover can 10x your development efficiency with agentic memory management and team collaboration features.
