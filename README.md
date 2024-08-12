<img align="left" width="200" src="guy.png">



# numbers guy
> 📈 a little CLI for fetching release and release asset data from GitHub

## Usage

```sh
Usage: numbers-guy [OPTIONS]

Options:
      --projects <PROJECTS>  A comma-separated list of projects in repo-slug format: `owner/repo`
  -h, --help                 Print help
```

For example, to get cargo-dist's and number-guy's download numbers you can run:

```sh
numbers-guy --projects axodotdev/cargo-dist,axodotdev/numbers-guy
```
