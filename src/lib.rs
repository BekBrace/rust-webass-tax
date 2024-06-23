// Import the wasm_bindgen prelude module for WebAssembly interaction.
use wasm_bindgen::prelude::*;

// Annotation to indicate that this function is accessible from JavaScript.
#[wasm_bindgen]
pub fn calculate_tax(income: f64) -> f64 {
    // Initialize the tax variable to 0.0.
    let mut tax = 0.0;

    // Check if income is less than or equal to $9,875.
    if income <= 9875.0 {
        // Calculate tax at 10% rate for income in this bracket.
        tax = income * 0.10;
    // Check if income is less than or equal to $40,125.
    } else if income <= 40125.0 {
        // Calculate tax for the first $9,875 at 10%, and the remainder at 12%.
        tax = 987.5 + (income - 9875.0) * 0.12;
    // Check if income is less than or equal to $85,525.
    } else if income <= 85525.0 {
        // Calculate tax for previous brackets and the remainder at 22%.
        tax = 4617.5 + (income - 40125.0) * 0.22;
    // Check if income is less than or equal to $163,300.
    } else if income <= 163300.0 {
        // Calculate tax for previous brackets and the remainder at 24%.
        tax = 14605.5 + (income - 85525.0) * 0.24;
    // Check if income is less than or equal to $207,350.
    } else if income <= 207350.0 {
        // Calculate tax for previous brackets and the remainder at 32%.
        tax = 33271.5 + (income - 163300.0) * 0.32;
    // Check if income is less than or equal to $518,400.
    } else if income <= 518400.0 {
        // Calculate tax for previous brackets and the remainder at 35%.
        tax = 47367.5 + (income - 207350.0) * 0.35;
    // For income above $518,400.
    } else {
        // Calculate tax for previous brackets and the remainder at 37%.
        tax = 156235.0 + (income - 518400.0) * 0.37;
    }

    // Return the calculated tax.
    tax
}
