# Norgmill

> ⚠️ **Work In Progress**: This project is under active development. Features may change significantly.

Renders norg files to html and runs a server to view files in browser. Why name it norgmill? I don't know, i am bad at naming, if you have a better name please suggest

# Usage

Install rust-toolchain using [rustup](https://rustup.rs/) or by system package manager.
```bash
git clone https://github.com/hardfau1t/norgmill.git
cd norgmill
cargo run -- serve -r <neorg workspace-folder> -v
```

# Customizing

Templates are currently stored in `./templates` folder and css files are present in `static` folder. You can write to these files to customize theme.

> 📝 **Note**: The project is still in alpha state, structure and file organization may change.


# Goals

- [ ] GTD workflow, at least a dashboard to view status of projects and tasks, next tasks etc
- [ ] 100% support for neorg specs
