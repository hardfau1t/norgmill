# Norganic

> ⚠️ **Work In Progress**: This project is under active development. Features may change significantly.

Renders norg files to html and runs a server to view files in browser

# Usage

Install rust-toolchain using [rustup](https://rustup.rs/) or by system package manager.
```bash
git clone https://github.com/hardfau1t/norganic.git
cd norganic
cargo run -- serve -r <neorg workspace-folder> -v
```

# Customizing

Templates are currently stored in `./templates` folder and css files are present in `static` folder. You can write to these files to customize theme.

> 📝 **Note**: The project is still in alpha state, structure and file organization may change.

