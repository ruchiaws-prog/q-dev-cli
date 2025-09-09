# Amazon Q CLI Demo - Example Output

This shows what users would see when running the demo command.

## Running `q demo`

```
🎯 Amazon Q CLI Demo
==================================================

Welcome to the Amazon Q CLI interactive demo!
This demo will showcase the key features of the Amazon Q CLI.

Would you like to start the demo? (y/N): y

💬 Chat Feature
--------------------
The q chat command starts an AI-powered chat session in your terminal.
You can ask questions about your code, get help with commands, and more!

Example usage:
  ❯ q chat
  ❯ q chat "How do I list files in Linux?"

Key features:
  • Natural language interaction
  • Context-aware responses
  • Code generation and explanation
  • Integration with your local environment
  • Real-time assistance with development tasks

Continue to next feature? (y/N): y

🔄 Translate Feature
--------------------
The q translate command converts natural language to shell commands.
Perfect for when you know what you want to do but not the exact command!

Example usage:
  ❯ q translate "find all python files"
  ❯ q ai "compress this folder"

Sample translations:
  "list all files larger than 100MB" → find . -size +100M -type f
  "show running processes" → ps aux
  "create a backup of config.json" → cp config.json config.json.bak

Continue to next feature? (y/N): y

⚙️  Settings Feature
--------------------
The q settings command helps you customize Amazon Q CLI behavior.
Configure themes, integrations, and personal preferences.

Example usage:
  ❯ q settings
  ❯ q settings all
  ❯ q settings open

What you can configure:
  • Appearance and themes
  • Shell integrations
  • Autocomplete behavior
  • Telemetry preferences
  • Custom shortcuts and aliases

Continue to next feature? (y/N): y

🩺 Doctor Feature
--------------------
The q doctor command diagnoses and fixes common installation issues.
Run this when something isn't working as expected!

Example usage:
  ❯ q doctor
  ❯ q doctor --all

What it checks:
  • Shell integration status
  • Permission issues
  • Configuration problems
  • Network connectivity
  • System compatibility

🚀 Next Steps
==================================================

Ready to get started? Here's what you can do:

1. q chat - Start chatting with Amazon Q
2. q translate "your command here" - Try natural language translation
3. q settings - Customize your experience
4. q doctor - Get help if you run into issues
5. q --help - See all available commands

For more information, visit: https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html

Happy coding! 🎉
```

## Running `q demo --auto`

Same content as above but without the interactive prompts - runs straight through all sections.

## Running `q demo --feature chat`

```
🎯 Amazon Q CLI Demo
==================================================

💬 Chat Feature
--------------------
The q chat command starts an AI-powered chat session in your terminal.
You can ask questions about your code, get help with commands, and more!

Example usage:
  ❯ q chat
  ❯ q chat "How do I list files in Linux?"

Key features:
  • Natural language interaction
  • Context-aware responses
  • Code generation and explanation
  • Integration with your local environment
  • Real-time assistance with development tasks

🚀 Next Steps
==================================================

Ready to get started? Here's what you can do:

1. q chat - Start chatting with Amazon Q
2. q translate "your command here" - Try natural language translation
3. q settings - Customize your experience
4. q doctor - Get help if you run into issues
5. q --help - See all available commands

For more information, visit: https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html

Happy coding! 🎉
```

## Running `q demo --help`

```
Interactive demo of Amazon Q CLI features

Usage: q demo [OPTIONS]

Options:
  -a, --auto                 Skip interactive prompts and run full demo
      --feature <FEATURE>    Show only a specific feature demo [possible values: chat, translate, settings, doctor]
  -h, --help                 Print help
```

## Integration with Main Help

The demo command now appears in the main help:

```
q (Amazon Q CLI)

Popular Subcommands              Usage: q [subcommand]
╭────────────────────────────────────────────────────╮
│ chat         Chat with Amazon Q                    │
│ translate    Natural Language to Shell translation │
│ demo         Interactive demo of CLI features       │
│ doctor       Debug installation issues             │ 
│ settings     Customize appearance & behavior       │
│ quit         Quit the app                          │
╰────────────────────────────────────────────────────╯

To see all subcommands, use:
 ❯ q --help-all
```