pub mod fibonacci {
    pub fn fibonacci(n: usize) -> usize {
        log::debug!("fibonacci({})", n);
        if n == 0 {
            log::trace!("n==0 => 0");
            0
        } else if n == 1 {
            log::trace!("n==1 => 1");
            1
        } else {
            log::trace!("n>=2 => fibonacci(n - 2) + fibonacci(n - 1)");
            fibonacci(n - 2) + fibonacci(n - 1)
        }
    }
}

pub mod tribonacci {
    pub fn tribonacci(n: usize) -> usize {
        log::debug!("tribonacci({})", n);
        if n < 2 {
            log::trace!("n<2 => 0");
            0
        } else if n == 2 {
            log::trace!("n==2 => 1");
            1
        } else {
            log::trace!("n>=3 => tribonacci(n-3) + tribonacci(n - 2) + tribonacci(n - 1)");
            tribonacci(n - 3) + tribonacci(n - 2) + tribonacci(n - 1)
        }
    }
}
