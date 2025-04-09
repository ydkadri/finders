# FindeRS
A tool to replace the complex bash `find` logic which searches for files (optionally) containing some string or regular expression pattern.

### The challenge
Finding files containing some string is a common use case in the shell, however the command is cumbersome:
```bash
# Bash command
find <dir> \
    -type f \
    -name <file pattern> \
    -exec grep -iH <search pattern> {} \;
```

Instead, `finders` provides a lightweight wrapper for this common command.

### Usage
```bash
Usage: finders [OPTIONS] [PATH]

Arguments:
  [PATH]  Optional path to operate on, defaults to CWD

Options:
  -f, --file-pattern <FILE_PATTERN>      File pattern to filter results
  -s, --search-pattern <SEARCH_PATTERN>  Search pattern to match in result files
  -r, --regex-pattern <REGEX_PATTERN>    Regex pattern to match in result files
  -i, --case-insensitive                 Flag for case insensitive search
  -v, --verbose                          Verbose output details unreadable files
  -h, --help                             Print help
  -V, --version                          Print version
```
