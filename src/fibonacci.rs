pub mod fibonacci {
    pub fn get_n_fibonacci(n: i32) -> i32 {
        //Generate the nth Fibonacci number.

        if n <= 0 {
          println!("The {}th Fibonacci number is {}", n, n);
            return n;
        }

        let mut previous_fib = 0;
        let mut current_fib = 1;
        let mut i = 0;

        let _n = n - 1;

        while i < _n {
            let new_fib = previous_fib + current_fib;
            previous_fib = current_fib;
            current_fib = new_fib;
            i += 1;
        }
        println!("The {}th Fibonacci number is {}", n, current_fib);
        return current_fib;
    }
}
