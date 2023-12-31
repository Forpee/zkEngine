# ZkEngine Test

Log trace of exection trace engine

## Getting Started

Follow these instructions to get the project up and running on your local machine.

### Prerequisites

Make sure you have the following installed:

- [Git](https://git-scm.com/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [Your preferred method to serve HTML locally (e.g., Live Server extension for VS Code)]

### Installation

```bash
# Clone the repository
git clone https://github.com/Forpee/zkEngine

# Navigate into the project directory
cd zkEngine

# Initialize and update the submodule
git submodule init
git submodule update

# Build the project with wasm-pack
wasm-pack build --target web
```
### Usage

1. Open `index.html` with your preferred method to serve HTML locally. If you're using VS Code, you can use the Live Server extension.

2. Enter the functions name in the "Function Name" input.

3. Enter the function's arguments separated by commas. For example, if your function is `add` and is typically called like `add(2, 4)`, you would input `2,4` in the input (only i32 args are accepted for now).

4.Then Enter the function's argument types separated by commas. For example, if your function is `add` and is typically called like `add(2: i32, 4.0: f32)`, you would input `i32,f32` in the input. (no spaces between values)

5. Click the "Choose File" button/input. Do this last as it triggers the `onchange` function. If you select a file first without entering the function name and arguments, it won't run.

Once the file is selected, you should see the execution trace of that WebAssembly (Wasm) file in the console.

### Example

In the `test` folder, you'll find examples wasm files. You can inspect what the wat looks like at [wasm2wat](https://webassembly.github.io/wabt/demo/wasm2wat/) to see the function argumets to input. Here are example inputs for the provided wasm files:

#### `add.wasm` Example:

- Function Name: 'add'
- Arguments: 2,4
- Argument types: i32,i32
- Selected File: add.wasm

#### `fibonacci.wasm` Example:

- Function Name: 'fibonacci'
- Arguments: 10
- Argument types: i32
- Selected File: fibonacci.wasm

#### `addi32_and_f32.wasm` Example:

- Function Name: 'addi32_and_f32'
- Arguments: 10,90.202
- Argument types: i32,f32
- Selected File: addi32_and_f32.wasm

