pub mod variables {
    pub fn variables_section() {
        example_one_variables_and_mutability();
        example_two_constants();
        example_three_shadowing();
        
    }
    fn example_one_variables_and_mutability() {
        println!("Intro To Variables:");
        print!("    ");
        println!("// let x = 5; // Variables are immutable by default");
        print!("    ");
        println!("// x = 42; // this will throw an error");
        print!("    ");
        println!("let mut x = 5; // adding mut makes it mutable");
        let mut x = 5;

        // printing the value of x
        print!("    ");
        println!("{} is the value of x", x);
        // x = 42; // this will throw an error
        print!("    ");
        println!("x = 42; // this will work!");
        x = 42; // this will work because x is mutable
        print!("    ");
        println!("{} is the value of x\n", x);
    }
    fn example_two_constants() {
        println!("Constants:");
        // Constants are always immutable
        print!("    ");
        println!("Constants are always immutable");
        // Constants can be declared in any scope, including the global scope
        print!("    ");
        println!("Constants can be declared in any scope, including the global scope");
        // Constants must be annotated with the type of their value
        print!("    ");
        println!("Constants must be annotated with the type of their value");
        // Constants can be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime
        print!("    ");
        println!("Constants can be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime\n");
        
        print!("    ");
        println!("const MAX_POINTS: u32 = 100_000;");
        const MAX_POINTS: u32 = 100_000;
        print!("    ");
        println!("{} is the value of MAX_POINTS\n", MAX_POINTS);
    }
    fn example_three_shadowing() {
        println!("Shadowing:");
        // Shadowing is a way to declare a new variable with the same name as a previous variable
        // Shadowing allows us to reuse the variable name rather than forcing us to create two unique variables
        // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword
        // Shadowing also allows us to change the type of the value but reuse the same name

        // Shadowing example
        
        let x = 420; // x is an i32 with value 420 
        //
        let x = x + 1; // x is an i32 with value 421 

        {
            let x = x * 2; // x is an i32 with value 842
            print!("    ");
            println!("{} is the value of x within the inner scope", x);
        }
        print!("    ");
        println!("{} is the value of x within the outer scope\n", x);
    }
}
