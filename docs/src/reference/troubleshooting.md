# Troubleshooting

Common issues and their solutions.

## Installation Issues

### "command not found: finder"

**Problem:** Shell can't find the finder binary.

**Solution:** Add the binary location to your PATH:

```bash
# Check where finder is installed
which finder

# If not found, add to PATH (in ~/.bashrc or ~/.zshrc)
export PATH="$PATH:/usr/local/bin"

# Or for cargo install
export PATH="$PATH:$HOME/.cargo/bin"

# Reload shell configuration
source ~/.bashrc  # or source ~/.zshrc
```

### Permission Denied

**Problem:** Binary is not executable.

**Solution:** Make it executable:

```bash
chmod +x /usr/local/bin/finder
```

### macOS "cannot be opened because the developer cannot be verified"

**Problem:** macOS Gatekeeper blocks unsigned binaries.

**Solution:**

```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /usr/local/bin/finder

# Or allow in System Preferences:
# System Preferences → Security & Privacy → General → Allow anyway
```

## Search Issues

### No Results When Expected

**Check 1: Verify the pattern**

```bash
# Test with simple, known pattern first
finder -s "README"

# Check case sensitivity
finder -s "pattern" -i
```

**Check 2: Verify file filtering**

```bash
# Remove file filter to search all files
finder -s "pattern"  # Without -f flag

# Check if files exist
finder -f ".rs" -l
```

**Check 3: Verify search location**

```bash
# Explicitly specify directory
finder /path/to/project -s "pattern"

# Check current directory
pwd
```

### Too Many Results

**Limit scope:**

```bash
# Search specific directory
finder src/ -s "pattern"

# Add file filter
finder -f ".rs" -s "pattern"

# Use more specific pattern
finder -r "^TODO:" -s "pattern"  # Line must start with TODO:
```

### Regex Not Working

**Problem:** Pattern not matching as expected.

**Common issues:**

```bash
# ❌ Wrong: shell interprets special characters
finder -r *.rs

# ✅ Correct: quote the pattern
finder -r ".*\.rs$"

# ❌ Wrong: mixing regex and literal search
finder -s -r "pattern"  # Can't use both

# ✅ Correct: choose one
finder -r "pattern"  # Regex
finder -s "pattern"  # Literal
```

**Test your regex:**

```bash
# Use a simple pattern first
finder -r "test"

# Add complexity gradually
finder -r "test.*function"
finder -r "test.*function.*\("
```

### Files Not Being Searched

**Check permissions:**

```bash
# Run with verbose flag
finder -s "pattern" -v

# This will show files that couldn't be read
```

**Common causes:**
- File is binary (only searches text files)
- Permission denied
- Symbolic link to non-existent file

## Output Issues

### No Colored Output

**Check 1: Verify terminal support**

```bash
# Force colors on
finder -s "pattern" --colour
```

**Check 2: Check environment variables**

```bash
# Check if colors are disabled
echo $NO_COLOR  # Should be empty

# Check CLICOLOR settings
echo $CLICOLOR
echo $CLICOLOR_FORCE
```

**Solution:**

```bash
# Enable colors
unset NO_COLOR

# Or force colors
finder -s "pattern" --colour
```

### Colored Output in Pipes/Files

**Problem:** Colors appear as escape codes when piping or redirecting.

**Solution:**

```bash
# Remove colors for pipes
finder -s "pattern" --no-colour > output.txt

# Or keep colors for less
finder -s "pattern" --colour | less -R
```

### JSON Output Invalid

**Problem:** JSON output is malformed.

**Check:**

```bash
# Validate JSON
finder -s "pattern" --json | jq .

# If jq fails, check for:
# - Binary files in output (shouldn't happen, but check with -v)
# - Special characters in filenames
```

**Solution:**

```bash
# Use verbose mode to identify problematic files
finder -s "pattern" --json -v
```

## Performance Issues

### Very Slow Search

**Check 1: Are you searching too many files?**

```bash
# Count files being searched
find . -type f | wc -l

# Limit scope
finder src/ -s "pattern"  # Instead of finder -s "pattern"
```

**Check 2: Is it a network drive?**

```bash
# Check mount points
df -h .

# Copy to local disk if on network drive
```

**Check 3: Large binary files?**

```bash
# Find large files
find . -type f -size +10M

# Exclude them from search
finder src/ -s "pattern"  # Instead of root directory
```

### High Memory Usage

**This shouldn't happen** - FindeRS uses streaming processing.

If you're seeing high memory usage:

1. Check if you're collecting all output in memory:
   ```bash
   # ❌ Bad: stores all results
   results=$(finder -s "pattern")
   
   # ✅ Good: process as stream
   finder -s "pattern" | while read line; do
     process "$line"
   done
   ```

2. Report as a bug with reproduction steps

## Error Messages

### "Permission denied"

**Cause:** Can't read file or directory.

**Solution:** Usually safe to ignore - these files are skipped. Use `-v` to see which files:

```bash
finder -s "pattern" -v
```

### "Invalid regex pattern"

**Cause:** Malformed regular expression.

**Solution:**

```bash
# Check regex syntax
finder -r "valid.*pattern"

# Escape special characters
finder -r "test\.rs"  # Literal dot
finder -r "\\$"       # Literal dollar sign

# Test pattern separately
echo "test string" | grep -E "your.*pattern"
```

### "No such file or directory"

**Cause:** Specified path doesn't exist.

**Solution:**

```bash
# Check path exists
ls /path/to/search

# Use relative or absolute path
finder ./src -s "pattern"
finder /absolute/path -s "pattern"
```

## Integration Issues

### Not Working in Shell Scripts

**Problem:** Works in terminal but not in scripts.

**Common causes:**

```bash
# ❌ PATH not set in script
#!/bin/bash
finder -s "pattern"  # May not find finder

# ✅ Use full path or set PATH
#!/bin/bash
export PATH="$PATH:/usr/local/bin"
finder -s "pattern"

# Or use full path
/usr/local/bin/finder -s "pattern"
```

### Not Working in Cron

**Problem:** Works manually but not in cron.

**Solution:** Cron has minimal environment:

```bash
# In crontab, set PATH
PATH=/usr/local/bin:/usr/bin:/bin

# Or use full path in command
0 9 * * * /usr/local/bin/finder /path/to/project -s "pattern"
```

### Not Working in Git Hooks

**Problem:** Hooks can't find finder.

**Solution:** Same as scripts - set PATH or use full path:

```bash
#!/bin/bash
# .git/hooks/pre-commit

PATH="/usr/local/bin:$PATH"
finder -s "NOCOMMIT" || exit 1
```

## Platform-Specific Issues

### Windows: Line Endings

**Problem:** Not finding patterns at line ends on Windows files.

**Cause:** Windows uses CRLF (`\r\n`), searches may not account for `\r`.

**Solution:**

```bash
# Convert line endings
find . -type f -name "*.txt" -exec dos2unix {} \;

# Or search for pattern with optional \r
finder -r "pattern\r?$"
```

### macOS: Spotlight Interference

**Problem:** Slow searches immediately after updating files.

**Cause:** Spotlight indexing in background.

**Solution:** Wait a moment, or:

```bash
# Exclude from Spotlight temporarily
sudo mdutil -i off /path/to/project
finder -s "pattern"
sudo mdutil -i on /path/to/project
```

## Getting Help

### Gathering Debug Information

When reporting issues, include:

```bash
# Version
finder --version

# System info
uname -a

# Command that failed (with -v flag)
finder -s "pattern" -v

# Environment
env | grep -E "(COLOR|PATH)"
```

### Where to Get Help

- **GitHub Issues:** https://github.com/ydkadri/finders/issues
- **Documentation:** https://ydkadri.github.io/finders
- **Email:** youcef.kadri@example.com

### Before Reporting a Bug

1. Check this troubleshooting guide
2. Try with latest version (`finder --version`)
3. Test with minimal example
4. Include reproduction steps

## Next Steps

- Review [Common Use Cases](../examples/common-use-cases.md) for usage patterns
- Check [CLI Reference](../cli-reference.md) for all options
- See [Performance](./performance.md) for optimization tips
