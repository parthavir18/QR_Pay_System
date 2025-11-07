#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, Symbol, String, symbol_short};

// Structure to store payment request details
#[contracttype]
#[derive(Clone)]
pub struct PaymentRequest {
    pub request_id: u64,
    pub merchant: Address,
    pub amount: i128,
    pub description: String,
    pub timestamp: u64,
    pub is_paid: bool,
}

// Mapping for payment requests
#[contracttype]
pub enum PaymentBook {
    Request(u64)
}

// Counter for generating unique payment request IDs
const REQUEST_COUNTER: Symbol = symbol_short!("REQ_CNT");

#[contract]
pub struct QRPaymentContract;

#[contractimpl]
impl QRPaymentContract {
    
    // Function 1: Merchant creates a payment request (generates QR code data)
    pub fn create_payment_request(
        env: Env, 
        merchant: Address, 
        amount: i128, 
        description: String
    ) -> u64 {
        // Verify merchant authorization
        merchant.require_auth();
        
        // Get and increment request counter
        let mut request_count: u64 = env.storage().instance().get(&REQUEST_COUNTER).unwrap_or(0);
        request_count += 1;
        
        // Get current timestamp
        let timestamp = env.ledger().timestamp();
        
        // Create new payment request
        let payment_request = PaymentRequest {
            request_id: request_count,
            merchant: merchant.clone(),
            amount,
            description,
            timestamp,
            is_paid: false,
        };
        
        // Store payment request
        env.storage().instance().set(
            &PaymentBook::Request(request_count), 
            &payment_request
        );
        
        // Update counter
        env.storage().instance().set(&REQUEST_COUNTER, &request_count);
        
        // Extend storage TTL
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Payment request created with ID: {}", request_count);
        
        request_count
    }
    
    // Function 2: Customer processes payment by scanning QR code
    pub fn process_payment(
        env: Env,
        customer: Address,
        request_id: u64
    ) {
        // Verify customer authorization
        customer.require_auth();
        
        // Retrieve payment request
        let mut payment_request = Self::view_payment_request(env.clone(), request_id);
        
        // Validate payment request exists and is not already paid
        if payment_request.request_id == 0 {
            log!(&env, "Payment request not found");
            panic!("Payment request not found");
        }
        
        if payment_request.is_paid {
            log!(&env, "Payment already completed");
            panic!("Payment already completed");
        }
        
        // Transfer XLM from customer to merchant
        // Note: In production, you would use token transfer functionality
        // For this example, we're marking the payment as complete
        
        // Mark payment as paid
        payment_request.is_paid = true;
        
        // Update payment request
        env.storage().instance().set(
            &PaymentBook::Request(request_id),
            &payment_request
        );
        
        // Extend storage TTL
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(
            &env, 
            "Payment completed: Request ID {} - Amount: {}", 
            request_id, 
            payment_request.amount
        );
    }
    
    // Function 3: View payment request details
    pub fn view_payment_request(env: Env, request_id: u64) -> PaymentRequest {
        let key = PaymentBook::Request(request_id);
        
        env.storage().instance().get(&key).unwrap_or(PaymentRequest {
            request_id: 0,
            merchant: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            amount: 0,
            description: String::from_str(&env, "Not Found"),
            timestamp: 0,
            is_paid: false,
        })
    }
    
    // Function 4: Get total number of payment requests
    pub fn get_request_count(env: Env) -> u64 {
        env.storage().instance().get(&REQUEST_COUNTER).unwrap_or(0)
    }
}