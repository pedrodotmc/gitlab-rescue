# gitlab-rescue

CLI tool for getting and importing GitLab CI/CD variables from a project (Read only).

```text
$ gitlab-rescue --help

gitlab-rescue 0.1.0
Pedro Miranda <pedrodotmc@gmail.com>
CLI tool for getting and importing GitLab CI/CD variables from a project (Read only).

USAGE:
    gitlab-rescue [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --group-id <GITLAB_GROUP_ID>
            GitLab group ID

    -p, --project-id <GITLAB_PROJECT_ID>
            GitLab project ID

    -t, --token <GITLAB_API_TOKEN>
            A valid GitLab API token

    -u, --url <GITLAB_URL>
            URL of GitLab API. Default: https://gitlab.com

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    get         Print variable in STDOUT
    list        List all variables in JSON format
    export      Export variable in current shell (if variable is File type, a file will be created and the path's file will be exported)
    env         Export all variables in current shell (file type variables will be stored in a folder)

Instead, you can set request parameters via environment variables:
export GITLAB_PROJECT_ID=<GITLAB_PROJECT_ID>
export GITLAB_API_TOKEN=<GITLAB_API_TOKEN>
export GITLAB_URL=<GITLAB_URL>
```

## gitlab-rescue get

```text
$ gitlab-rescue get --help

gitlab-rescue-get 0.1.0
Pedro Miranda <pedrodotmc@gmail.com>
Print variable in STDOUT

USAGE:
    gitlab-rescue get --name <VARIABLE_NAME> [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information

    --from-all-if-missing
            If variable is not found in defined environment (-e option), try searching in "All" environment.

OPTIONS:
    -e, --environment <ENVIRONMENT>
            Name of GitLab CI/CD environment (Default: All)

    -n, --name <NAME>
            Name of GitLab CI/CD variable
```

## gitlab-rescue list

```text
$ gitlab-rescue list --help

gitlab-rescue-list 0.1.0
Pedro Miranda <pedrodotmc@gmail.com>
List GitLab CI/CD variables in JSON format (by default first 20 variables).

USAGE:
    gitlab-rescue list [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information

    -a, --all
            List all varibles. By default, this command only load first 20 variables (https://docs.gitlab.com/ee/api/README.html#offset-based-pagination).

    --from-all-if-missing
            If variable is not found in defined environment (-e option), try searching in "All" environment.

OPTIONS:
    -e, --environment <ENVIRONMENT>
            Name of GitLab CI/CD environment (Default: All)
        
        --page <PAGE>
            Page number (See https://docs.gitlab.com/ee/api/README.html#offset-based-pagination). Default: 1.

        --per-page <PER_PAGE>
            Number of items to list per page (See https://docs.gitlab.com/ee/api/README.html#offset-based-pagination). Default: 20, Max. 100.
```

## gitlab-rescue export

```text
$ gitlab-rescue export --help

gitlab-rescue-export 0.1.0
Pedro Miranda <pedrodotmc@gmail.com>
Export variable in current shell (if variable is File type, a file will be created and the path's file will be exported)

USAGE:
    gitlab-rescue export --name <NAME> [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information
    
    --from-all-if-missing
            If variable is not found in defined environment (-e option), try searching in "All" environment.

OPTIONS:
    -e, --environment <ENVIRONMENT>
            Name of GitLab CI/CD environment (Default: All)

    -n, --name <NAME>
            Name of GitLab CI/CD variable

    -o, --output-file <FILE>
            Path file when value will be stored (only for variables with type "File"). Default: $PWD/<NAME>.var
```

## gitlab-rescue export-all

```text
$ gitlab-rescue env --help

gitlab-rescue-env 0.1.0
Pedro Miranda <pedrodotmc@gmail.com>
Export variables in current shell (by default first 20 variables).

USAGE:
    gitlab-rescue export-all [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information

    -a, --all
            List all varibles. By default, this command only load first 20 variables (https://docs.gitlab.com/ee/api/README.html#offset-based-pagination).

        --from-all-if-missing
            If variables are not found in defined environment (-e option), try searching in "All" environment.

OPTIONS:
    -e, --environment <ENVIRONMENT>
            Name of GitLab CI/CD environment. (Default: None)

        --folder <PATH>
            Path where variables with type "File" will be stored. Files will be created with format <VARIABLE_NAME>.var. Default: $PWD/.env.<ENVIRONMENT>.
        
        --page <PAGE>
            Page number (See https://docs.gitlab.com/ee/api/README.html#offset-based-pagination). Default: 1.

        --per-page <PER_PAGE>
            Number of items to list per page (See https://docs.gitlab.com/ee/api/README.html#offset-based-pagination). Default: 20, Max. 100.
```

## Usage

```bash
# Instead of using CLI flags, you can export GitLab instance variables
$ export GITLAB_PROJECT=<GITLAB_PROJECT>
$ export GITLAB_API_TOKEN=<GITLAB_API_TOKEN>
$ export GITLAB_URL=<GITLAB_URL>

# Get a variable
$ gitlab-rescue get -n MY_VARIABLE
hello-world

# Get a file
$ gitlab-rescue get -n MY_CREDENTIALS -e develop
{
    "a_super_secret_info": "a_super_secret_value"
}

# Export a variable
$ gitlab-rescue export -n MY_VARIABLE
$ echo $MY_VARIABLE
hello-world

# Export a file
$ gitlab-rescue export -n MY_CREDENTIALS -e develop
$ echo $MY_CREDENTIALS
$PWD/MY_CREDENTIALS.var
$ cat $MY_CREDENTIALS
{
    "a_super_secret_info": "a_super_secret_value"
}

# Export all
$ gitlab-rescue export-all -e develop
$ echo $MY_VARIABLE ## This variable is not in "develop" scope, so it was not exported.
$ echo $MY_CREDENTIALS
$PWD/.env.develop/MY_CREDENTIALS.var
$ cat $MY_CREDENTIALS
{
    "a_super_secret_info": "a_super_secret_value"
}

# Export all with fallback
$ gitlab-rescue export-all -e develop --from-all-if-missing
$ echo $MY_VARIABLE
hello-world
$ echo $MY_CREDENTIALS
$PWD/.env.develop/MY_CREDENTIALS.var
$ cat $MY_CREDENTIALS
{
    "a_super_secret_info": "a_super_secret_value"
}

# List variables
$ gitlab-rescue list -e develop --from-all-if-missing >output.json
$ cat output.json
{
    "MY_VARIABLE": "",
    "MY_CREDENTIALS": "{\\n\"a_super_secret_info\": \"a_super_secret_value\"\\n}"
}

# For example, you can get a JSON file using jq as follows:
$ jq '.MY_CREDENTIALS | fromjson' output.json >my_credentials.json
$ cat my_credentials.json
{
    "a_super_secret_info": "a_super_secret_value"
}
```
