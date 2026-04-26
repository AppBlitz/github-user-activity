# GitHub User Activity

## Installation

### Clone repository

```bash
git clone https://github.com/AppBlitz/github-user-activity.git
cd github-user-activity
```

### Build project with Cargo

```bash
cargo build --release
```

### Run executable

```bash
./target/release/github-user-activity
```

---

## Usage

### Flags

| Short | Full                  | Description                        |
|-------|-----------------------|------------------------------------|
| `-n`  | `--name-repository`   | Name of the repository to query    |
| `-u`  | `--user-name-github`  | GitHub profile username            |
| `-c`  | `--creation-pdf`      | Generate a PDF with the results    |



## Examples

### Query a user's repository activity
 
#### Using full flag

##### name repository

```bash
./target/release/github-user-activity --name-repository  <name_perfil_github>/<name_repositoryy> --user-name-github <username_github>
```

##### creation pdf

1. **Option 1**
```bash
./target/release/github-user-activity --creation-pdf --user-name-github <username_github>
```

2. **Option 2**
```bash
./target/release/github-user-activity --creation-pdf --name-repository <name_perfil_github>/<name_repository>  --user-name-github <username_github>
```

#### Using short flag

##### name repository

```bash
./target/release/github-user-activity -n <name_perfil_github>/<name_repository> -u <username_github>
```

##### creation pdf

1. **Option 1**
```bash
./target/release/github-user-activity -c -u <username_github>
```

2. **Option 2**
```bash
./target/release/github-user-activity -c -n <name_perfil_github>/<name_repository>  -u <username_github>
```
