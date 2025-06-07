# ByteRover MCP Server

This extension provides a Model Context Server for ByteRover, for use with the Zed AI assistant.

## Configuration

To use the extension, you will need to config the context server by setting the `memory_workspace_key` and `user_email` in your Zed `<project-root/.zed/settings.json`

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

## Usage

- `create-memories`: Create a new memory by capturing key coding insights from human-agent interactions to help solve similar tasks in the future.
- `search-memories`: Search a curated knowledge base of past coding solutions and insights to find relevant help for your current task.


## Rule

```text
[mcp-server-byterover]
# important
always use search-memories tool to get the related context before any tasks
always use create-memories to store all the critical informations after sucessfull tasks
```
