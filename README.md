## You Know
Sync your github or gitlab repositories description with your local git description, and display a list of
directories with their descriptions from the command line

## Usage
```
USAGE:
    yn [OPTIONS] [PATH]

ARGS:
    <PATH>

OPTIONS:
    -a, --all        Do not ignore entries starting with a dot
    -h, --help       Print help information
    -l, --list       Display files as list
    -s, --sync       Sync GitHub and GitLab repository description with git description
    -V, --version    Print version information
```

## Examples
```
# Downlaod descriptions from github/gitlab into ./.git/description file
$ yn -s
```


```
# Download descriptions and reveal files (and hidden files) as a list
$ yn -lsa

prototype, Repository not linked with origin
awesome_project, No description
SoupGB, A gameboy emulator written in rust
MyDocuments
```

- If a directory is a repository, but the repository has no remote pointing to github/gitlab, then you'll see
    "Repository not linked with origin"
- If a directory is a repository but there's no description in github/gitlab, you'll see "No description"
- If a directory is a repository and there's a description in github/gitlab, it will be saved in ./.git/description
    and you'll see it every time you run `yn -l` or `yn -la`
- If your directory is not a repository, you won't see any text after the directory name
