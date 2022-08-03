/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{near_bindgen, env, Promise, AccountId};

mod students;
mod course;

use students::Student;
use course::Course;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    students: LookupMap<AccountId, Student>,
    courses: Vector<Course>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            students: LookupMap::new(b"s"),
            courses: Vector::new(b"c"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /**
    * Adding data to a LookupMap
    */
    #[payable]
    pub fn add_to_map(&mut self, name: String, course: String, year: u32, age: u32) {
        // Get wallet address
        let signer: AccountId = env::predecessor_account_id();

        // Get attached deposit
        let attached_deposit: u128 = env::attached_deposit();

        // Get initial storage
        let initial_storage = env::storage_usage();

        // Creating student object
        let student: Student = Student {
            name,
            course,
            year,
            age,
        };

        // Store student object
        self.students.insert(&signer, &student);

        self.pay_for_storage(initial_storage, attached_deposit);
    }
    
    /**
    * Getting data from LookupMap
    */
    pub fn get_from_map(&self) -> Option<Student> {
        // Get wallet address
        let signer = env::predecessor_account_id();

        // Retreive student object
        match self.students.get(&signer) {
            Some(student) => Some(student),
            None => None,
        }
    }
    
    /**
    * Deleting data from LookupMap
    */
    pub fn remove_from_map(&mut self) -> Option<Student> {
        // Get wallet address
        let signer = env::predecessor_account_id();

        // Remove student object
        let student = self.students.remove(&signer);
        student
    }
    
    /**
    * Searching data in LookupMap
    */
    pub fn search_in_map(&self, account_id: AccountId) -> bool {
        // Search for the key
        if self.students.contains_key(&account_id) {
            true
        } else {
            false
        }
    }
    
    /**
    *   Adding data into a Vector
    */
    pub fn add_to_vec(&mut self, name: String) {
        // Code goes here
        let course = Course { name };

        // Add course to courses
        self.courses.push(&course);
    }
    
    /**
    * Getting data from a Vector
    */
    pub fn get_from_vec(&self, start: u32, limit: u32) -> Vec<Course> {
        // Code goes here
        let range = u32::min(limit, self.courses.len() as u32) + start;

        let mut result: Vec<Course> = vec![];

        for index in start..range {
            let course: Course = self.courses.get(index.into()).unwrap();
            result.push(course);
        }

        result
    }
    
    /**
    * Deleting data from a Vector
    */
    pub fn remove_from_vec(&mut self, index: u32) -> Course {
        // Code goes here
        let course = self.courses.swap_remove(index.into());
        course
    }
    
    /**
    * Storage Staking: Is the delegation of storage cost to users (respective origins of data). 
    * This way users pay for the storage of every data they add to the storage in a smart contract.
    */
    fn pay_for_storage(&self, initial_storage: u64, attached_storage_cost: u128) {
        // Get Current Storage
        let current_storage = env::storage_usage();
        
        // Get Storage Used
        let storage_used = current_storage - initial_storage;
        
        // Get Storage cost per byte
        let storage_cost: u128 = env::storage_byte_cost();
        
        // Get payable storage fee
        if let Some(total_storage_cost) = storage_cost.checked_mul(storage_used as u128) {
            // Check if user attached enough tokens to cater for storage
            assert!(attached_storage_cost >= total_storage_cost, "Insufficient funds!");
            
            // Check for balance
            let excess_balance = attached_storage_cost - total_storage_cost;
            if excess_balance > 0 {
                // Return excess tokens to user
                self.return_excess_tokens(excess_balance);
            }
        }
    }
    
    /**
    * Sends back excess tokens to user
    */
    pub fn return_excess_tokens(&self, excess_balance: u128) {
        // Get signer address
        let signer = env::predecessor_account_id();
        
        // Send back excess
        Promise::new(signer).transfer(excess_balance);
    }
}