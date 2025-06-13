# ByteRover MCP Server

This extension provides a Model Context Server for ByteRover, for use with the Zed AI assistant.

ByteRover is a self-improving, shared memory layer that remembers your AI agent's coding experiences—enabling smarter, faster development and seamless collaboration across your entire dev team. [Learn more about ByteRover](https://www.byterover.dev/?source=zed).

## Configuration

To use the extension:

1. Visit [ByteRover](https://www.byterover.dev/?source=zed) to create a memory workspace
2. Follow the Memory workspace quickstart guide to get your configuration values
3. Configure the context server by setting the `memory_workspace_key` and `user_email` in your Zed `<project-root>/.zed/settings.json`

```json
{
  "context_servers": {
    "mcp-server-byterover": {
      "settings": {
        "memory_workspace_key": "MEMORY_WORKSPACE_KEY",
        "user_email": "YOUR_EMAIL"
      }
    }
  }
}
```

## Memory Tools

The ByteRover MCP server provides two core memory management tools:

- **`search-memories`**: Search your curated knowledge base of past coding solutions and insights to find relevant help for your current task
- **`create-memories`**: Capture key coding insights from human-agent interactions to help solve similar tasks in the future

## Rule System for Memory Management

The ByteRover extension uses a flexible rule system that allows you to customize how the AI agent interacts with your memory workspace. Rules define **when** and **how** the agent should use memory operations during development workflows.

### Basic Configuration


```text
# Important: Automated memory workflow
always use search-memories tool to get the related context before any tasks
always use create-memories to store all the critical informations after sucessfull tasks
```

This configuration ensures the agent:
1. **Searches existing memories** before starting any development task to leverage past solutions
2. **Stores new insights** after successfully completing tasks to build your knowledge base

### Customizing Memory Behavior

#### When to Use Memory

You can configure different triggers for memory operations:

```text
# Search memory before specific tasks
always use search-memories before implementing new features
always use search-memories when debugging complex issues
always use search-memories for architecture decisions

# Store memories based on task completion
always use create-memories after successful bug fixes
always use create-memories when completing code reviews
always use create-memories for performance optimizations
```

#### How to Use Memory

Create a `<project-root>/.rules` file and add your custom rules to control how the agent uses memory:

```text
# Context-specific memory usage
use search-memories for similar technology stacks only
use create-memories to store reusable patterns not full implementations
focus create-memories on design decisions and lessons learned

# Memory format guidelines
store coding patterns and architectures in create-memories
capture algorithm explanations and trade-offs in create-memories
include dependency recommendations and gotchas in create-memories
```

#### Advanced Memory Workflows

```text
# Conditional memory usage
use search-memories when working with unfamiliar libraries
use search-memories before refactoring legacy code

# Team collaboration
use search-memories to maintain coding standards across team
store create-memories with clear problem-solution mapping
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
