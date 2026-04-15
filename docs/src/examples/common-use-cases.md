# Common Use Cases

Real-world examples of how to use FindeRS effectively.

## Development Workflows

### Find TODOs and FIXMEs

Track technical debt across your codebase:

```bash
finder -s "TODO" -l
finder -s "FIXME" -c
finder -r "TODO|FIXME" -i
```

### Find Configuration Files

Locate all config files in a project:

```bash
finder -f "config"
finder -f ".json" -s "database"
finder -f ".yaml" -s "api"
```

### Search for Function Definitions

Find function definitions across different languages:

```bash
# Rust functions
finder -f ".rs" -r "fn \w+\("

# Python functions
finder -f ".py" -r "def \w+\("

# JavaScript functions
finder -f ".js" -r "function \w+\("
```

## Code Review and Refactoring

### Find All Usages

Locate all references to a variable or function:

```bash
finder -s "old_function_name"
finder -f ".rs" -s "OldStruct"
```

### Find Deprecated APIs

Search for deprecated API usage:

```bash
finder -s "deprecated" -i
finder -r "@deprecated|DEPRECATED"
```

### Find Error Handling

Review error handling patterns:

```bash
finder -s "unwrap()"
finder -s "expect("
finder -r "panic!|unwrap|expect"
```

## Security and Compliance

### Find Sensitive Data

**⚠️ Be careful not to commit findings!**

```bash
# Find potential API keys
finder -r "[A-Za-z0-9]{32,}" -i

# Find passwords in config
finder -s "password" -i -f ".env"

# Find secrets
finder -r "secret|token|key" -i
```

### Audit Logging

Find logging statements:

```bash
finder -s "log::" -f ".rs"
finder -r "console\.(log|error|warn)"
```

## Documentation

### Find Undocumented Code

Locate public APIs without documentation:

```bash
# Rust public items without docs
finder -f ".rs" -r "pub (fn|struct|enum|trait) \w+" -l | \
  xargs -I {} sh -c 'grep -B1 "pub" {} | grep -q "///" || echo {}'
```

### Find Broken Links

Search for old URLs or references:

```bash
finder -s "oldcompany.com"
finder -s "http://" -f ".md"
```

## Testing

### Find Test Coverage Gaps

Locate modules without tests:

```bash
# Find modules without test modules
finder -f ".rs" -l | while read f; do
  grep -q "#\[cfg(test)\]" "$f" || echo "$f"
done
```

### Find Disabled Tests

Locate ignored or disabled tests:

```bash
finder -s "#[ignore]"
finder -s "skip_test" -i
```

## Data Processing

### Count Patterns Across Files

Get statistics about pattern usage:

```bash
# Total TODO count
finder -s "TODO" -c | awk -F: '{sum+=$2} END {print sum}'

# Files with most errors
finder -s "error" -c | sort -t: -k2 -nr | head
```

### Extract Structured Data

Pull specific data from files:

```bash
# Extract all email addresses
finder -r "[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}" --json | \
  jq -r '.[].matches[].content'

# Extract version numbers
finder -s "version" --json | jq
```

## Integration with Other Tools

### Open Files in Editor

```bash
# Edit all files with TODOs
vim $(finder -s "TODO" -l)

# Open files with errors
code $(finder -s "ERROR" -l)
```

### Pipe to Other Commands

```bash
# Count lines in matched files
finder -f ".rs" -l | xargs wc -l

# Find largest files
finder -f ".log" -l | xargs ls -lh | sort -k5 -hr
```

### Create Reports

```bash
# Generate TODO report
finder -s "TODO" > todos.txt

# JSON report for CI
finder -s "FIXME" --json > fixmes.json
```

## Performance Optimization

### Find Large Files

Identify files that might slow down searches:

```bash
finder -f ".log" -l | xargs ls -lh | awk '$5 ~ /M|G/'
```

### Profile Pattern Complexity

Test search performance:

```bash
time finder -r "complex.*regex.*pattern"
time finder -s "simple string"
```

## Next Steps

- Check [Advanced Patterns](./advanced-patterns.md) for more complex use cases
- See [Integration Examples](./integration.md) for CI/CD workflows
- Review [CLI Reference](../cli-reference.md) for all available options
