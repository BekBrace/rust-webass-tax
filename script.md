<!-- Hey bla bla bla ... -->

# Build a simple web application using Rust and Web Assembly.

Summary
------
- 1- Introduction to WebAssembly: Understand what WebAssembly is and how it works with Rust and JavaScript.

- 2- Setting Up the Project: Create a new Rust project and configure it for WebAssembly.

- 3 - Writing the Rust Code: Implement the tax calculation logic in Rust.

- 4 - Building the Project: Use wasm-pack to build the project.

- 5 - Setting Up the Web Environment: Create an HTML file to load the WebAssembly module and provide a user interface.

- 6 - Serving the Project: Use a simple HTTP server to serve the project and test it in a web browser.

<!-- --------------------------------------------------- -->
<!-- --------------------------------------------------- -->
<!-- --------------------------------------------------- -->

# Introduction to WebAssembly

# What is WebAssembly?

WebAssembly (often abbreviated as Wasm) is a binary instruction format for a stack-based virtual machine. 

It is designed as a portable compilation target for programming languages, enabling high-performance applications to run on web pages. 

WebAssembly is intended to execute at near-native speed by taking advantage of common hardware capabilities.

# How WebAssembly Works with Rust and JavaScript ?

What It Is: WebAssembly is a binary instruction format designed for high-performance execution and designed to be a compilation target for high-level languages like C, C++, and Rust. This means you write your code in these languages and compile it into WebAssembly, which can then run in web browsers.

How It Works: WebAssembly runs in a sandboxed execution environment, providing a secure and fast way to execute code. It works alongside JavaScript, allowing web developers to leverage the performance of Wasm for compute-intensive tasks while using JavaScript for the rest of the application.

WebAssembly modules can be written in languages like Rust and then executed in web environments alongside JavaScript. Rust, known for its performance and safety, is a great fit for writing WebAssembly modules. JavaScript can then be used to interact with these modules, providing a seamless integration between high-performance logic and dynamic web applications.

<!-- -------------------------------------------------- -->

Creating a WebAssembly Tax Calculator with Rust
Step 1: Setting Up the Project
Make sure you have Rust installed - Check my Rust courses, or go directly to rust website to download and install Rust, but I think you already have it installed, so let's move on

# Create a Rust Project
cargo new tax-calc-wasm
cd tax-calc-wasm

- Update Cargo.toml: Open Cargo.toml and add the following under [dependencies] and [lib] sections to configure the project for Wasm:
```toml
[dependencies]
wasm-bindgen = "0.2"
wasm-bindgen is a crate that facilitates the interaction between Rust and JavaScript when targeting WebAssembly. 

[lib]
'lib'
This section of Cargo.toml is used to configure settings for the Rust library we are creating.

crate-type: This specifies the type of output that the Rust compiler should produce. In Rust, a "crate" is a compilation unit, and crates can be either binary (executables) or libraries.
"cdylib": This stands for "C-compatible dynamic library." When targeting WebAssembly, this type indicates that we want to produce a dynamic library that can be used in environments expecting C-compatible calling conventions. Essentially, it tells the Rust compiler to generate a shared library suitable for WebAssembly.

simpler words:
------------
In the context of WebAssembly, specifying crate-type = ["cdylib"] ensures that the Rust compiler produces output that can be used as a WebAssembly module, which can then be loaded and executed in a web browser or other JavaScript environments.

In summary:

1 wasm-bindgen = "0.2": Adds the wasm-bindgen crate as a dependency, which provides the tools needed for Rust to communicate with JavaScript through WebAssembly.

2 [lib] crate-type = ["cdylib"]: Configures the Rust project to produce a C-compatible dynamic library, suitable for use as a WebAssembly module.
These configurations are crucial for building a Rust project that can be compiled to WebAssembly and used in web applications, enabling seamless integration between Rust and JavaScript.



```

# Step 2: Writing the Rust Code
Edit src/lib.rs to implement the tax calculation logic:
```rust
// Importing wasm_bindgen crate to enable communication between JavaScript and Rust
use wasm_bindgen::prelude::*;

// Define the function that will be exposed to JavaScript
#[wasm_bindgen]
pub fn calculate_tax(income: f64) -> f64 {
    // Initialize the tax variable
    let mut tax = 0.0;

    // Apply tax brackets to calculate the tax amount
    if income <= 9875.0 {
        tax = income * 0.10;
    } else if income <= 40125.0 {
        tax = 987.5 + (income - 9875.0) * 0.12;
    } else if income <= 85525.0 {
        tax = 4617.5 + (income - 40125.0) * 0.22;
    } else if income <= 163300.0 {
        tax = 14605.5 + (income - 85525.0) * 0.24;
    } else if income <= 207350.0 {
        tax = 33271.5 + (income - 163300.0) * 0.32;
    } else if income <= 518400.0 {
        tax = 47367.5 + (income - 207350.0) * 0.35;
    } else {
        tax = 156235.0 + (income - 518400.0) * 0.37;
    }

    // Return the calculated tax
    tax
}
```

# Step 3: Building the Project
Build the project using wasm-pack:
```bash
wasm-pack build --target web
```

# Step 4: Setting Up the Web Environment
Create an index.html file in the tax-calculator-wasm directory with the following content:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Tax Calculator</title>
    <style>
        /* Basic styling for the body */
        body {
            font-family: Arial, sans-serif;
            background-color: #f7f7f7;
            color: #333;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            margin: 0;
        }
        /* Styling for the main heading */
        h1 {
            color: #d35400; /* Rust color */
        }
        /* Margins for labels, inputs, and buttons */
        label, input, button {
            margin: 10px 0;
        }
        /* Styling for the input field */
        input {
            padding: 8px;
            font-size: 16px;
            border: 2px solid #d35400; /* Rust color */
            border-radius: 4px;
        }
        /* Styling for the button */
        button {
            padding: 10px 20px;
            font-size: 16px;
            background-color: #e67e22; /* Orange color */
            color: #fff;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            transition: background-color 0.3s ease;
        }
        /* Hover effect for the button */
        button:hover {
            background-color: #d35400; /* Darker rust color */
        }
        /* Styling for the result text */
        #result {
            margin-top: 20px;
            font-size: 18px;
            color: #e67e22; /* Orange color */
        }
    </style>
</head>
<body>
    <!-- Main heading of the page -->
    <h1>Tax Calculator</h1>
    <!-- Label and input for the income value -->
    <label for="income">Income: </label>
    <input type="number" id="income" name="income" min="0">
    <!-- Button to trigger tax calculation -->
    <button id="calculateButton">Calculate Tax</button>
    <!-- Paragraph to display the result -->
    <p id="result"></p>
    <script type="module">
        // Importing the WebAssembly module and the calculate_tax function
        import init, { calculate_tax } from "./pkg/tax_calculator_wasm.js";

        // Function to initialize the WebAssembly module
        async function run() {
            await init(); // Wait for the module to be initialized

            // Function to calculate tax when the button is clicked
            function calculateTax() {
                const income = parseFloat(document.getElementById("income").value); // Get the income value from the input field
                const tax = calculate_tax(income); // Call the calculate_tax function from the WebAssembly module
                document.getElementById("result").innerText = `Tax: $${tax.toFixed(2)}`; // Display the result
            }

            // Attach the calculateTax function to the button's click event
            document.getElementById("calculateButton").addEventListener("click", calculateTax);
        }

        // Run the initialization function
        run();
    </script>
</body>
</html>
```

# Step 5: Serving the Project

To serve the project, you need a simple web server.
Install a Web Server: You can install one using npm:
## npm install -g http-server

http-server .
Navigate to http://localhost:8000 in your web browser to see the tax calculator in action.
