# VS Code Integration Guide for rust_orm_gen

This guide explains how to set up and use VS Code for developing the `rust_orm_gen` project with full IDE support.

## 🚀 Quick Start

1. **Open the project in VS Code:**
   ```bash
   code rust_orm_gen.code-workspace
   ```

2. **Install recommended extensions** when prompted, or manually install:
   - Rust Analyzer
   - CodeLLDB
   - crates
   - Even Better TOML

## 📁 VS Code Configuration Files

### `.vscode/settings.json`
- **Rust Analyzer settings** for optimal Rust development
- **Code formatting** and linting configuration
- **File associations** for Rust files

### `.vscode/tasks.json`
- **Build tasks** for Cargo operations
- **Test tasks** for running tests
- **Custom tasks** for project-specific operations

### `.vscode/launch.json`
- **Debugging configuration** for the main binary
- **LLDB integration** for Rust debugging
- **Breakpoint support** and variable inspection

### `.vscode/extensions.json`
- **Recommended extensions** for Rust development
- **Automatic installation** prompts
- **Consistent development environment**

### `rust_orm_gen.code-workspace`
- **Workspace-level settings** and configuration
- **Multi-folder support** for complex projects
- **Shared VS Code settings** across team members

## 🛠️ Available Tasks

### Build Tasks
- **`cargo build`** - Build the project
- **`cargo check`** - Check for compilation errors
- **`cargo test`** - Run all tests
- **`cargo clippy`** - Run Clippy linter

### Development Tasks
- **`cargo run`** - Run the main binary
- **`cargo run -- visualize`** - Run visualization commands
- **`cargo run -- migrate`** - Run database migrations

### Testing Tasks
- **`cargo test`** - Run all tests
- **`cargo test --lib`** - Run library tests only
- **`cargo test --bin`** - Run binary tests only

## 🐛 Debugging

### Launch Configuration
1. Set breakpoints in your Rust code
2. Press `F5` or use Run → Start Debugging
3. Choose "Debug rust_orm_gen" configuration
4. Use LLDB commands in the Debug Console

### Debug Features
- **Breakpoint support** with conditional breakpoints
- **Variable inspection** and watch expressions
- **Call stack** navigation
- **Step through** code execution

## 🔧 Rust Analyzer Features

### Code Intelligence
- **Auto-completion** for Rust syntax and crates
- **Go to definition** and references
- **Hover information** for types and functions
- **Error highlighting** and quick fixes

### Refactoring
- **Rename symbols** across the codebase
- **Extract functions** and methods
- **Organize imports** automatically
- **Format code** on save

## 📊 Project-Specific Features

### Database Schema Visualization
- **Generate diagrams** directly from VS Code
- **Preview HTML** visualizations in browser
- **Export formats** (SVG, PDF, Visio) from tasks

### Database Operations
- **Run migrations** from VS Code terminal
- **Generate ORM code** with build tasks
- **Schema introspection** and validation

## ⌨️ Keyboard Shortcuts

### Rust Development
- `Ctrl+Shift+P` → "Rust Analyzer: Reload Workspace"
- `F12` → Go to definition
- `Shift+F12` → Find all references
- `Ctrl+Space` → Trigger suggestions

### Debugging
- `F5` → Start debugging
- `F9` → Toggle breakpoint
- `F10` → Step over
- `F11` → Step into
- `Shift+F11` → Step out

### General
- `Ctrl+Shift+P` → Command palette
- `Ctrl+Shift+E` → Explorer
- `Ctrl+Shift+X` → Extensions
- `Ctrl+Shift+G` → Source control

## 🔍 Code Navigation

### File Explorer
- **Project structure** with Rust file icons
- **Search files** with `Ctrl+P`
- **Go to symbol** with `Ctrl+T`

### Rust Analyzer
- **Workspace symbols** search
- **Implementations** of traits
- **Type hierarchy** navigation
- **Call hierarchy** analysis

## 📝 Code Quality

### Linting
- **Clippy integration** for additional checks
- **Error highlighting** in real-time
- **Quick fixes** for common issues
- **Custom lint rules** support

### Formatting
- **rustfmt integration** for code formatting
- **Format on save** option
- **Custom formatting** rules
- **Consistent code style** across team

## 🧪 Testing Integration

### Test Explorer
- **Run individual tests** from the test tree
- **Test results** in the Test panel
- **Debug tests** with breakpoints
- **Test coverage** reporting

### Test Commands
- **Run all tests** with `Ctrl+Shift+P` → "Test: Run All Tests"
- **Run failed tests** only
- **Run tests in file** with `Ctrl+Shift+P` → "Test: Run Tests in Current File"

## 🔌 Extension Recommendations

### Essential Extensions
- **rust-lang.rust-analyzer** - Rust language support
- **vadimcn.vscode-lldb** - Debugging support
- **serayuzgur.crates** - Cargo.toml dependency management
- **tamasfe.even-better-toml** - TOML file support

### Optional Extensions
- **ms-vscode.vscode-json** - JSON schema support
- **bradlc.vscode-tailwindcss** - CSS support for HTML generation
- **ms-vscode.powershell** - PowerShell support on Windows
- **ms-vscode.vscode-typescript-next** - TypeScript support for web features

## 🚀 Performance Tips

### Rust Analyzer Optimization
- **Exclude target directory** from analysis
- **Limit workspace size** for large projects
- **Use workspace settings** for consistent configuration
- **Restart analyzer** if performance degrades

### VS Code Performance
- **Disable unused extensions** in workspace
- **Use workspace-specific settings** when possible
- **Limit file watching** for large projects
- **Regular workspace reloads** for fresh state

## 🐛 Troubleshooting

### Common Issues
1. **Rust Analyzer not working** → Reload workspace
2. **Build errors** → Check Cargo.toml and dependencies
3. **Debugging not working** → Verify LLDB installation
4. **Extensions not loading** → Check VS Code version compatibility

### Debug Commands
- **Developer: Reload Window** → `Ctrl+Shift+P`
- **Rust Analyzer: Reload Workspace** → `Ctrl+Shift+P`
- **Developer: Toggle Developer Tools** → `F12`

## 📚 Additional Resources

### Documentation
- [Rust Analyzer User Manual](https://rust-analyzer.github.io/manual.html)
- [VS Code Rust Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [CodeLLDB Documentation](https://github.com/vadimcn/codelldb)

### Community
- [Rust Analyzer GitHub](https://github.com/rust-analyzer/rust-analyzer)
- [VS Code Rust Community](https://github.com/rust-lang/vscode-rust)
- [Rust Discord](https://discord.gg/rust-lang)

## 🎯 Enhanced VS Code Integration Features

### ✨ What's New
- **Comprehensive settings.json** with optimized Rust Analyzer configuration
- **Enhanced debugging configurations** with multiple launch targets
- **Custom keybindings** for common development tasks
- **Code snippets** for Rust ORM development patterns
- **Extended task definitions** with watch mode and coverage

### 🔧 New Configuration Files
- `.vscode/settings.json` - Optimized IDE settings for Rust development
- `.vscode/keybindings.json` - Custom keyboard shortcuts
- `.vscode/rust.code-snippets` - Code snippets for ORM patterns
- Enhanced `.vscode/launch.json` - Multiple debugging configurations
- Enhanced `.vscode/extensions.json` - Extended extension recommendations

### ⌨️ Custom Keyboard Shortcuts
- `Ctrl+Shift+B` - Build project
- `Ctrl+Shift+T` - Run tests  
- `Ctrl+Shift+R` - Run project
- `Ctrl+Shift+V` - Generate schema visualizations
- `Ctrl+Shift+C` - Run Clippy linter

### 📝 Code Snippets Available
- `derive_serde` - Add Serde derive macros
- `derive_debug` - Add Debug and Clone derives
- `result_fn` - Function returning Result type
- `async_fn` - Async function template
- `db_model` - Database model structure
- `test_fn` - Test module and function
- `async_test` - Async test function
- `sql_query` - SQL query builder pattern
- `migration` - Database migration template

### 🚀 Enhanced Tasks
- **Watch Mode** - Auto-rebuild on file changes
- **Test Coverage** - Generate HTML coverage reports
- **Code Formatting** - Format code with rustfmt
- **Browser Integration** - Auto-open HTML visualizations

## 🎯 Next Steps

1. **Install recommended extensions**
2. **Restart VS Code** to apply all configurations
3. **Test custom keybindings** and tasks
4. **Try code snippets** in Rust files
5. **Set up debugging** for your workflow
6. **Share workspace** with team members

---

*This integration guide is part of the rust_orm_gen project. For more information, see the main README.md and PROJECT_INDEX.md files.*
