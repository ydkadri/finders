# Advanced Patterns

Complex workflows and advanced techniques for power users.

## Regex Patterns

### Character Classes and Quantifiers

```bash
# Find phone numbers
finder -r "\d{3}-\d{3}-\d{4}"

# Find hex colors
finder -r "#[0-9a-fA-F]{6}"

# Find email addresses
finder -r "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"
```

### Lookahead and Lookbehind

```bash
# Find TODO comments with ticket numbers
finder -r "TODO: \w+-\d+"

# Find functions with specific parameters
finder -r "fn \w+\([^)]*&str[^)]*\)"
```

### Word Boundaries

```bash
# Find exact word matches
finder -r "\berror\b"

# Find variable names
finder -r "\b[a-z_][a-z0-9_]*\b"
```

## Combining Multiple Searches

### Boolean Logic

```bash
# Files with pattern A OR pattern B
finder -r "pattern_a|pattern_b"

# Multiple patterns (run separate searches)
finder -f ".rs" -s "TODO"
finder -f ".md" -s "TODO"
```

## Working with Structured Output

### Processing JSON Output

```bash
# Extract file paths only
finder -s "error" --json | jq -r '.[].path'

# Count matches per file
finder -s "TODO" --json | jq '.[] | {path: .path, count: (.matches | length)}'

# Filter by line number
finder -s "error" --json | jq '.[] | select(.matches[].line > 100)'
```

### Building Custom Reports

```bash
# Create CSV report
finder -s "TODO" --json | jq -r '.[] | .matches[] | [.line, .content] | @csv' > report.csv

# Generate HTML report
echo "<html><body><ul>" > report.html
finder -s "TODO" -l | while read f; do
  echo "<li><a href='$f'>$f</a></li>" >> report.html
done
echo "</ul></body></html>" >> report.html
```

## Shell Integration

### Custom Aliases

Add to your `.bashrc` or `.zshrc`:

```bash
# Find and edit
alias fe='vim $(finder -l)'

# Find todos in current project
alias todos='finder -s "TODO" -c'

# Search with context
alias search='finder -s'
```

### Functions

```bash
# Find and replace across files
find-replace() {
  local pattern="$1"
  local replacement="$2"
  finder -s "$pattern" -l | xargs sed -i "s/$pattern/$replacement/g"
}

# Count pattern occurrences
count-pattern() {
  finder -s "$1" -c | awk -F: '{sum+=$2} END {print sum}'
}
```

## Performance Optimization

### Limiting Search Scope

```bash
# Search specific directory
finder src/ -s "pattern"

# Search specific file types in directory
finder src/ -f ".rs" -s "pattern"

# Search only recently modified files
find . -mtime -7 -type f | xargs finder -s "pattern"
```

### Parallel Processing

```bash
# Process results in parallel with xargs
finder -s "pattern" -l | xargs -P 4 -I {} sh -c 'process {}'

# GNU parallel for complex operations
finder -s "pattern" -l | parallel 'complex-operation {}'
```

## CI/CD Integration

### GitHub Actions

```yaml
- name: Check for TODOs
  run: |
    if finder -s "TODO" -l > /dev/null; then
      echo "Found TODOs in code"
      finder -s "TODO"
      exit 1
    fi
```

### GitLab CI

```yaml
check-todos:
  script:
    - finder -s "TODO" > todos.txt
    - test ! -s todos.txt
  artifacts:
    paths:
      - todos.txt
    when: on_failure
```

### Pre-commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

if finder -s "NOCOMMIT" > /dev/null; then
  echo "Error: Found NOCOMMIT markers"
  finder -s "NOCOMMIT"
  exit 1
fi
```

## Code Quality Checks

### Detect Anti-patterns

```bash
# Find unwrap() in Rust
finder -f ".rs" -s "unwrap()" -c

# Find console.log in production JS
finder -f ".js" -s "console.log" | grep -v "test"

# Find SQL injection risks
finder -r "execute.*\+.*\$"
```

### Complexity Metrics

```bash
# Find deeply nested code
finder -r "^\s{12,}" -c

# Count function definitions
finder -f ".rs" -r "fn \w+\(" -c
```

## Documentation Generation

### Extract API Documentation

```bash
# Extract all doc comments
finder -f ".rs" -r "///.*" --json | \
  jq -r '.[] | .matches[] | .content' > api-docs.txt
```

### Generate Index

```bash
# Create module index
finder -f ".rs" -r "pub mod \w+" | \
  sed 's/.*pub mod /- /' > modules.md
```

## Debugging and Diagnostics

### Trace Code Flow

```bash
# Find all log statements
finder -r "(log|debug|info|warn|error)::" -f ".rs"

# Find panic locations
finder -s "panic!" -f ".rs"
```

### Dependency Analysis

```bash
# Find external crate usage
finder -r "use \w+::" -f ".rs" | sort | uniq

# Find module dependencies
finder -r "mod \w+;" -f ".rs"
```

## Working with Large Codebases

### Incremental Search

```bash
# Search one directory at a time
for dir in src/*/ ; do
  echo "Searching $dir"
  finder "$dir" -s "pattern"
done
```

### Caching Results

```bash
# Cache file list for repeated searches
finder -f ".rs" -l > rust-files.txt
cat rust-files.txt | xargs finder -s "pattern"
```

## Next Steps

- Review [Common Use Cases](./common-use-cases.md) for everyday patterns
- See [Integration Examples](./integration.md) for CI/CD workflows
- Check [Performance](../reference/performance.md) for optimization tips
