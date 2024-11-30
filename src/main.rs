use std::io;

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Payment {
    payment_number: usize,
    principal_payment: f64,
    interest_payment: f64,
    total_payment: f64,
    remaining_balance: f64,
}

impl Display for Payment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Payment {}:\n\tPrincipal: ${:.2}\n\tInterest: ${:.2}\n\tTotal: ${:.2}\n\tRemaining Balance: ${:.2}\n",
            self.payment_number,
            self.principal_payment,
            self.interest_payment,
            self.total_payment,
            self.remaining_balance
        )
    }
}

fn calculate_amortization_schedule(
    principal: f64,
    interest_rate: f64,
    term_in_months: usize,
) -> Vec<Payment> {
    let monthly_interest_rate = interest_rate / 12.0 / 100.0;
    let monthly_payment =
        principal * monthly_interest_rate * f64::powf(1.0 + monthly_interest_rate, term_in_months as f64)
            / (f64::powf(1.0 + monthly_interest_rate, term_in_months as f64) - 1.0);

    let mut remaining_balance = principal;
    let mut schedule = Vec::new();

    for month in 1..=term_in_months {
        let interest_payment = remaining_balance * monthly_interest_rate;
        let principal_payment = monthly_payment - interest_payment;
        remaining_balance -= principal_payment;

        schedule.push(Payment {
            payment_number: month,
            principal_payment,
            interest_payment,
            total_payment: monthly_payment,
            remaining_balance,
        });
    }

    schedule
}

fn main() {
    println!("Enter the principal amount:");
    let mut principal_input = String::new();
    io::stdin()
        .read_line(&mut principal_input)
        .expect("Failed to read input");
    let principal: f64 = principal_input.trim().parse().unwrap();

    println!("Enter the annual interest rate (in %):");
    let mut interest_rate_input = String::new();
    io::stdin()
        .read_line(&mut interest_rate_input)
        .expect("Failed to read input");
    let interest_rate: f64 = interest_rate_input.trim().parse().unwrap();

    println!("Enter the loan term in years:");
    let mut term_in_years_input = String::new();
    io::stdin()
        .read_line(&mut term_in_years_input)
        .expect("Failed to read input");
    let term_in_years: usize = term_in_years_input.trim().parse().unwrap();

    let term_in_months = term_in_years * 12;

    let schedule = calculate_amortization_schedule(principal, interest_rate, term_in_months);

    println!("Amortization Schedule:");
    for payment in schedule {
        println!("{}", payment);
    }
}