# Integration Examples

Integrate FindeRS into your development workflows, CI/CD pipelines, and automation scripts.

## Continuous Integration

### GitHub Actions

#### Check for Forbidden Patterns

```yaml
name: Code Quality Checks

on: [push, pull_request]

jobs:
  check-patterns:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install FindeRS
        run: |
          wget https://github.com/ydkadri/finders/releases/latest/download/finder-x86_64-linux.tar.gz
          tar -xzf finder-x86_64-linux.tar.gz
          sudo mv finder /usr/local/bin/
      
      - name: Check for debug statements
        run: |
          if finder -s "console.log" -l -f ".js" > /dev/null; then
            echo "❌ Found console.log statements"
            finder -s "console.log" -f ".js"
            exit 1
          fi
      
      - name: Check for TODOs
        run: |
          finder -s "TODO" -c > todo-count.txt
          cat todo-count.txt
```

#### Generate Reports

```yaml
      - name: Generate TODO Report
        run: |
          echo "# TODO Report" > todo-report.md
          echo "" >> todo-report.md
          finder -s "TODO" --json | \
            jq -r '.[] | "## \(.path)\n\(.matches[] | "- Line \(.line): \(.content)")\n"' \
            >> todo-report.md
      
      - name: Upload Report
        uses: actions/upload-artifact@v4
        with:
          name: todo-report
          path: todo-report.md
```

### GitLab CI

```yaml
code-quality:
  stage: test
  image: ubuntu:latest
  before_script:
    - apt-get update && apt-get install -y wget
    - wget https://github.com/ydkadri/finders/releases/latest/download/finder-x86_64-linux.tar.gz
    - tar -xzf finder-x86_64-linux.tar.gz
    - mv finder /usr/local/bin/
  script:
    - finder -s "FIXME" > fixmes.txt
    - test ! -s fixmes.txt || (cat fixmes.txt && exit 1)
  artifacts:
    paths:
      - fixmes.txt
    when: on_failure
```

### CircleCI

```yaml
version: 2.1

jobs:
  code-quality:
    docker:
      - image: ubuntu:latest
    steps:
      - checkout
      - run:
          name: Install FindeRS
          command: |
            apt-get update && apt-get install -y wget
            wget https://github.com/ydkadri/finders/releases/latest/download/finder-x86_64-linux.tar.gz
            tar -xzf finder-x86_64-linux.tar.gz
            mv finder /usr/local/bin/
      - run:
          name: Check patterns
          command: |
            finder -s "TODO" -c
```

## Git Hooks

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Running pre-commit checks..."

# Check for NOCOMMIT markers
if finder -s "NOCOMMIT" > /dev/null; then
  echo "❌ Error: Found NOCOMMIT markers"
  finder -s "NOCOMMIT"
  exit 1
fi

# Check for debug statements
if finder -s "debugger" -f ".js" > /dev/null; then
  echo "❌ Error: Found debugger statements"
  finder -s "debugger" -f ".js"
  exit 1
fi

# Check for unwrap() in Rust (warning only)
unwrap_count=$(finder -s "unwrap()" -f ".rs" -c | awk -F: '{sum+=$2} END {print sum}')
if [ "$unwrap_count" -gt 0 ]; then
  echo "⚠️  Warning: Found $unwrap_count unwrap() calls"
fi

echo "✅ Pre-commit checks passed"
```

### Pre-push Hook

```bash
#!/bin/bash
# .git/hooks/pre-push

echo "Running pre-push checks..."

# Generate security report
if finder -r "password|secret|api_key" -i --json > security-scan.json; then
  match_count=$(jq '[.[] | .matches | length] | add // 0' security-scan.json)
  if [ "$match_count" -gt 0 ]; then
    echo "⚠️  Warning: Found $match_count potential security issues"
    echo "Review security-scan.json before pushing"
  fi
fi

echo "✅ Pre-push checks complete"
```

## Editor Integration

### VS Code Tasks

Create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Find TODOs",
      "type": "shell",
      "command": "finder -s TODO",
      "problemMatcher": [],
      "presentation": {
        "reveal": "always",
        "panel": "new"
      }
    },
    {
      "label": "Find in Files",
      "type": "shell",
      "command": "finder",
      "args": [
        "-s",
        "${input:searchPattern}"
      ],
      "problemMatcher": []
    }
  ],
  "inputs": [
    {
      "id": "searchPattern",
      "type": "promptString",
      "description": "Enter search pattern"
    }
  ]
}
```

### Vim Integration

Add to `.vimrc`:

```vim
" Search with FindeRS and populate quickfix
command! -nargs=1 FindeRS cexpr system('finder -s ' . shellescape(<q-args>))

" Find word under cursor
nnoremap <leader>f :FindeRS <C-R><C-W><CR>

" Find TODOs
nnoremap <leader>t :FindeRS TODO<CR>
```

## Shell Scripts

### Batch Processing

```bash
#!/bin/bash
# process-matches.sh

# Find all files matching pattern and process them
finder -s "$1" -l | while read file; do
  echo "Processing $file..."
  # Your processing logic here
  process_file "$file"
done
```

### Report Generation

```bash
#!/bin/bash
# generate-report.sh

OUTPUT_FILE="code-quality-report.html"

cat > "$OUTPUT_FILE" << 'EOF'
<!DOCTYPE html>
<html>
<head>
  <title>Code Quality Report</title>
  <style>
    body { font-family: sans-serif; margin: 2em; }
    .section { margin: 2em 0; }
    .count { font-weight: bold; color: #d73a49; }
  </style>
</head>
<body>
  <h1>Code Quality Report</h1>
EOF

echo "<div class='section'>" >> "$OUTPUT_FILE"
echo "<h2>TODOs</h2>" >> "$OUTPUT_FILE"
TODO_COUNT=$(finder -s "TODO" -c | awk -F: '{sum+=$2} END {print sum}')
echo "<p class='count'>Total: $TODO_COUNT</p>" >> "$OUTPUT_FILE"
echo "<pre>" >> "$OUTPUT_FILE"
finder -s "TODO" >> "$OUTPUT_FILE"
echo "</pre>" >> "$OUTPUT_FILE"
echo "</div>" >> "$OUTPUT_FILE"

echo "</body></html>" >> "$OUTPUT_FILE"

echo "Report generated: $OUTPUT_FILE"
```

## Make Integration

```makefile
.PHONY: check-todos check-fixmes check-patterns

check-todos:
	@echo "Checking for TODOs..."
	@finder -s "TODO" -c || true

check-fixmes:
	@echo "Checking for FIXMEs..."
	@finder -s "FIXME" -c || true

check-patterns: check-todos check-fixmes
	@echo "Checking for unwrap()..."
	@finder -f ".rs" -s "unwrap()" -c || true

check: check-patterns
	@echo "All checks complete"
```

## Docker Integration

### Dockerfile

```dockerfile
FROM rust:latest as builder

# Install FindeRS
RUN wget https://github.com/ydkadri/finders/releases/latest/download/finder-x86_64-linux.tar.gz && \
    tar -xzf finder-x86_64-linux.tar.gz && \
    mv finder /usr/local/bin/

# Use in build steps
RUN finder -s "TODO" -c
```

### Docker Compose

```yaml
version: '3'
services:
  code-quality:
    image: ubuntu:latest
    volumes:
      - .:/workspace
    working_dir: /workspace
    command: >
      bash -c "
        apt-get update && apt-get install -y wget &&
        wget https://github.com/ydkadri/finders/releases/latest/download/finder-x86_64-linux.tar.gz &&
        tar -xzf finder-x86_64-linux.tar.gz &&
        mv finder /usr/local/bin/ &&
        finder -s TODO -c
      "
```

## Monitoring and Alerting

### Track Technical Debt

```bash
#!/bin/bash
# track-debt.sh

# Count TODOs and FIXMEs over time
DATE=$(date +%Y-%m-%d)
TODO_COUNT=$(finder -s "TODO" -c | awk -F: '{sum+=$2} END {print sum}')
FIXME_COUNT=$(finder -s "FIXME" -c | awk -F: '{sum+=$2} END {print sum}')

echo "$DATE,$TODO_COUNT,$FIXME_COUNT" >> debt-tracking.csv

# Alert if count exceeds threshold
if [ "$TODO_COUNT" -gt 100 ]; then
  echo "⚠️  Warning: TODO count exceeded 100!"
  # Send alert (email, Slack, etc.)
fi
```

### Slack Integration

```bash
#!/bin/bash
# slack-alert.sh

WEBHOOK_URL="your-slack-webhook-url"

TODO_COUNT=$(finder -s "TODO" -c | awk -F: '{sum+=$2} END {print sum}')

curl -X POST "$WEBHOOK_URL" \
  -H 'Content-Type: application/json' \
  -d "{
    \"text\": \"Daily Code Quality Report\",
    \"blocks\": [
      {
        \"type\": \"section\",
        \"text\": {
          \"type\": \"mrkdwn\",
          \"text\": \"*TODO Count:* $TODO_COUNT\"
        }
      }
    ]
  }"
```

## Next Steps

- Explore [Common Use Cases](./common-use-cases.md) for more patterns
- Check [Advanced Patterns](./advanced-patterns.md) for complex workflows
- Review [Troubleshooting](../reference/troubleshooting.md) for common issues
