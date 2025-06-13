# Rinf Framework Workflow Guidelines

## **Critical Workflow Rules**

### 🚫 **DO NOT Edit Generated Files**
- **NEVER** manually create or edit `.dart` files in `lib/src/`
- These are **auto-generated** by Rinf

### ⚡ **Always Run Code Generation**
- **ALWAYS** run `rinf gen` after creating/modifying Rust signal types
- This generates the corresponding Dart bindings automatically

### 📁 **File Organization**
- **Rust signals**: Define in `native/hub/src/signals/`
- **Custom Dart files**: Create in `lib/` (same level as `main.dart`)
- **Generated files**: Located in `lib/src/` (hands-off!)

### 🔧 **Signal Type Requirements**
Must derive one of these traits for code generation:
- `DartSignal` - for Dart → Rust communication
- `RustSignal` - for Rust → Dart communication
- `SignalPiece` - for nested signal components

### 📋 **Workflow Steps**
1. Define signal structs/enums in Rust with appropriate derive macros
2. Run `rinf gen` in terminal
3. Use generated Dart classes in your Flutter code
4. Import from `src/bindings/bindings.dart`

### ⚠️ **Remember**
The generated Dart files handle all serialization, deserialization, and signal routing automatically. Focus on your business logic, not the plumbing!