#[allow(unused_imports)]
// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId,Promise,log, near_bindgen};
use near_sdk::serde::{Serialize,Deserialize};
use std::collections::HashMap; 



#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,Debug,PartialEq,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Freelancer {
    pub account_id: AccountId,
    pub profile_image: String,
    pub full_name: String,
    pub hourly_rate: u128,
    pub profession: String,
    pub payment_preference: String,
    pub skills: Vec<String>,
    pub profile_rating: u128,
    pub is_profile_public: bool,
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,Debug,PartialEq,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Dispute {
    pub dispute_in_job_id:u128,
    pub dispute_id:u128,
    pub dispute_name: String,
    pub description: String,
    pub disputor: String,
    pub client_involved: String,
    pub resolved: bool,
    
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,Debug,PartialEq,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FreelancerPortfolio {
    pub account_id: AccountId,
    pub images: Vec<String>,
    pub videos: Vec<String>,
    pub task_url: String,
    pub description: String,
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,Debug,PartialEq,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FreelancerExperience {
    pub account_id: AccountId,
    pub from_date: String,
    pub to_date: String,
    pub job_title: String,
    pub job_description: String,
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,Debug,PartialEq,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ClientJobs {
    pub job_id: u128,
    pub account_id: AccountId,
    pub project_title: String,
    pub project_description: String,
    pub project_duration: String,
    pub project_budget: u128,
    pub skill_requirements: Vec<String>,
    pub images: Vec<String>,
    pub bid_available: bool,
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,Debug,PartialEq,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FreelancerBid {
    pub job_id: u128,
    pub bid_id: u128,
    pub account_id: AccountId,
    pub bid_description: String,
    pub budget: u128,
    pub relevant_files: Vec<String>,
    pub bid_approved: bool,
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,PartialEq,Debug,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectMilestone {
    pub job_id:u128,
    pub bid_id: u128,
    pub milestone_id: u128,
    pub milestone_name: String,
    pub milestone_description: String,
    pub milestone_budget: u128,
    pub milestone_duration: u128,
    pub milestone_work_approved: bool,
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,Debug,PartialEq,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Chat {
    pub timestamp: u128,
    pub sender: AccountId,
    pub receiver: AccountId,
    pub message: String,
    pub attached_files: Vec<String>,
    pub seen: bool,
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,PartialEq,Debug,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ClientRatings {
    pub rating_id: u128,
    pub project_name: String,
    pub account_id: AccountId,
    pub rating_user:AccountId,
    pub feedback: String,
    pub rating_for_timely_payments: u128,
    pub rating_for_timely_feedbacks: u128,
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,PartialEq,Debug,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FreelancerRating {
    pub rating_id: u128,
    pub project_name: String,
    pub account_id: AccountId,
    pub rating_user:AccountId,
    pub feedback: String,
    pub rating_for_completed_projects: u128,
    pub rating_for_communication_skills: u128,
}

#[derive(Clone,BorshDeserialize, BorshSerialize, Serialize,Debug,PartialEq,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Transaction {
    pub from: AccountId,
    pub to: AccountId,
    pub transaction_purpose: String,
    pub transaction_amount: u128,
    pub timestamp: u128,
    pub transaction_status: String,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct KaziNear{
  freelancers: HashMap<AccountId, Freelancer>,
  disputes: HashMap<u128, Dispute>,
  freelancer_portfolios: HashMap<AccountId, FreelancerPortfolio>,
  freelancer_experiences: HashMap<AccountId, FreelancerExperience>,
  client_jobs: HashMap<u128, ClientJobs>,
  freelancer_bids: HashMap<u128, FreelancerBid>,
  project_milestones: HashMap<u128, ProjectMilestone>,
  chats: HashMap<AccountId, Chat>,
  client_ratings: HashMap<AccountId, ClientRatings>,
  freelancer_ratings: HashMap<AccountId, FreelancerRating>,
  transactions: HashMap<AccountId, Transaction>,
  commission_fee:u128,
}


impl Default for KaziNear{
  fn default() -> Self {
    Self{
      freelancers: HashMap::new(),
      disputes:HashMap::new(),
      freelancer_portfolios:HashMap::new(),
      freelancer_experiences:HashMap::new(),
      client_jobs:HashMap::new(),
      freelancer_bids:HashMap::new(),
      project_milestones:HashMap::new(),
      chats:HashMap::new(),
      client_ratings:HashMap::new(),
      freelancer_ratings:HashMap::new(),
      transactions:HashMap::new(),
      commission_fee:2,
    }
  }
}

#[near_bindgen]
impl KaziNear {

      // Freelancer

        // Create a new freelancer entry
        pub fn create_freelancer(&mut self, account_id: AccountId, freelancer: Freelancer) {
          self.freelancers.insert(account_id, freelancer);
      }

      // Read a freelancer's information
      pub fn get_freelancer(&self, account_id: AccountId) -> Option<Freelancer> {
          self.freelancers.get(&account_id).cloned()
      }

      // Update a freelancer's information
      pub fn update_freelancer(&mut self, account_id: AccountId, updated_freelancer: Freelancer) {
          self.freelancers.insert(account_id, updated_freelancer);
      }

      // Delete a freelancer's information
      pub fn delete_freelancer(&mut self, account_id: AccountId) {
          self.freelancers.remove(&account_id);
      }

      // Get the total number of freelancers
      pub fn get_total_freelancers(&self) -> usize {
          self.freelancers.len()
      }


      // Freelancer Portfolio

      pub fn create_freelancer_portfolio(&mut self, account_id: AccountId, portfolio: FreelancerPortfolio) {
        self.freelancer_portfolios.insert(account_id, portfolio);
    }

      // Read a freelancer portfolio
      pub fn get_freelancer_portfolio(&self, account_id: AccountId) -> Option<FreelancerPortfolio> {
          self.freelancer_portfolios.get(&account_id).cloned()
      }

      // Update a freelancer portfolio entry
      pub fn update_freelancer_portfolio(&mut self, account_id: AccountId, updated_portfolio: FreelancerPortfolio) {
          self.freelancer_portfolios.insert(account_id, updated_portfolio);
      }

      // Delete a freelancer portfolio entry
      pub fn delete_freelancer_portfolio(&mut self, account_id: AccountId) {
          self.freelancer_portfolios.remove(&account_id);
      }

      // Freelancer Experience

    // Create a new freelancer experience entry
      pub fn create_freelancer_experience(&mut self, account_id: AccountId, experience: FreelancerExperience) {
        self.freelancer_experiences.insert(account_id, experience);
        }

        // Read a freelancer experience entry
        pub fn get_freelancer_experience(&self, account_id: AccountId) -> Option<FreelancerExperience> {
            self.freelancer_experiences.get(&account_id).cloned()
        }

        // Update a freelancer experience entry
        pub fn update_freelancer_experience(&mut self, account_id: AccountId, updated_experience: FreelancerExperience) {
            self.freelancer_experiences.insert(account_id, updated_experience);
        }

        // Delete a freelancer experience entry
        pub fn delete_freelancer_experience(&mut self, account_id: AccountId) {
            self.freelancer_experiences.remove(&account_id);
        }


       //  Dispute 


        // Create a new dispute entry
        pub fn create_dispute(&mut self, dispute_id: u128, dispute: Dispute) {
            self.disputes.insert(dispute_id, dispute);
        }

        // Read a dispute entry
        pub fn get_dispute(&self, dispute_id: u128) -> Option<Dispute> {
            self.disputes.get(&dispute_id).cloned()
        }

        // Update a dispute entry
        pub fn update_dispute(&mut self, dispute_id: u128, updated_dispute: Dispute) {
            self.disputes.insert(dispute_id, updated_dispute);
        }

        // Delete a dispute entry
        pub fn delete_dispute(&mut self, dispute_id: u128) {
            self.disputes.remove(&dispute_id);
        }

        pub fn get_disputes_by_job_id(&self, job_id: u128) -> Vec<Dispute> {
            let mut result = Vec::new();
            for dispute in self.disputes.values() {
                if dispute.dispute_in_job_id == job_id {
                    result.push(dispute.clone());
                }
            }
            
            // Sort the vector by dispute_id
            result.sort_by(|a, b| a.dispute_id.cmp(&b.dispute_id));
            
            result
        }

    // Client Jobs

    // Create a new job entry
        #[payable]
        pub fn create_client_job(
            &mut self,
            job_id: u128,
            job: ClientJobs,
        ) {
                // Ensure the attached deposit is equal to or greater than the job's budget
                assert!(
                env::attached_deposit() >= job.project_budget,
                "Attached deposit must be equal to or greater than the job's budget"
            );
            self.client_jobs.insert(job_id, job);
        }

        // Read a job entry
        pub fn get_client_job(&self, job_id: u128) -> Option<ClientJobs> {
            self.client_jobs.get(&job_id).cloned()
        }

        // Update a job entry
        pub fn update_client_job(
            &mut self,
            job_id: u128,
            updated_job: ClientJobs,
        ) {
            self.client_jobs.insert(job_id, updated_job);
        }

        // Delete a job entry
        pub fn delete_client_job(&mut self, job_id: u128) {
            self.client_jobs.remove(&job_id);
        }

        // Get all jobs for a specific account
        pub fn get_all_jobs_for_account(&self, account_id: AccountId) -> Vec<ClientJobs> {
            let mut result = Vec::new();
            for job in self.client_jobs.values() {
                if job.account_id == account_id {
                    result.push(job.clone());
                }
            }
            result
        }

        // Get all client jobs
        pub fn get_all_client_jobs(&self) -> Vec<ClientJobs> {
            let mut result = self.client_jobs.values().cloned().collect::<Vec<ClientJobs>>();
            
            // Sort the vector by job_id
            result.sort_by(|a, b| a.job_id.cmp(&b.job_id));
            
            result
        }



      // Chats

      // Create a new chat entry
        pub fn create_chat(&mut self, sender_id: AccountId,  chat: Chat) {
            self.chats.insert(sender_id, chat);
        }

        // Read a chat entry
        pub fn get_chat(&self, sender_id: AccountId) -> Option<Chat> {
        
            self.chats.get(&sender_id).cloned()
        }

        // Update a chat entry
        pub fn update_chat(&mut self, sender_id: AccountId,updated_chat: Chat) {
            self.chats.insert(sender_id, updated_chat);
        }

        // Delete a chat entry
        pub fn delete_chat(&mut self, sender_id: AccountId) {
            self.chats.remove(&sender_id);
        }

        // Get all chats for a specific account
        // Get all chats for a specific account
        pub fn get_all_chats_for_account(&self, account_id: AccountId) -> Vec<Chat> {
        let mut result = Vec::new();
        for chat in self.chats.values() {
            if chat.receiver == account_id || chat.sender == account_id {
                result.push(chat.clone());
            }
        }
        result.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        result
        }


    // Client Rating

        // Create a new client rating entry
        pub fn create_client_rating(&mut self, account_id: AccountId,rating: ClientRatings) {
            self.client_ratings.insert(account_id, rating);
        }

        // Read a client rating entry by account ID
        pub fn get_client_rating_by_account(&self, account_id: AccountId) -> Option<ClientRatings> {
            self.client_ratings.get(&account_id).cloned()
        }

            // Get client ratings for a specific user (by account_id)
        pub fn get_client_ratings_by_account(&self, account_id: AccountId) -> Option<Vec<ClientRatings>> {
            self.client_ratings.get(&account_id).map(|rating| vec![rating.clone()])
        }

        // Update a client rating entry by account ID
        pub fn update_client_rating_by_account(&mut self, account_id: AccountId, updated_rating: ClientRatings) {
            self.client_ratings.insert(account_id, updated_rating);
        }

        // Delete a client rating entry by account ID
        pub fn delete_client_rating_by_account(&mut self, account_id: AccountId) {
            self.client_ratings.remove(&account_id);
        }

        // Calculate and return the average rating for timely payments for a specific user
        pub fn get_average_rating_for_timely_payments_by_account(&self, account_id: AccountId) -> Option<u128> {
            let user_ratings: Vec<u128> = self
                .client_ratings
                .values()
                .filter(|rating| rating.account_id == account_id)
                .map(|rating| rating.rating_for_timely_payments)
                .collect();

            if user_ratings.is_empty() {
                None
            } else {
                Some(user_ratings.iter().sum::<u128>() / user_ratings.len() as u128)
            }
        }


        // Calculate and return the average rating for timely feedbacks for a specific user
        pub fn get_average_rating_for_timely_feedbacks_by_account(&self, account_id: AccountId) -> Option<u128> {
            let user_ratings: Vec<u128> = self
                .client_ratings
                .get(&account_id)
                .map(|rating| vec![rating.rating_for_timely_feedbacks])
                .unwrap_or_else(|| Vec::new());

            if user_ratings.is_empty() {
                None
            } else {
                Some(user_ratings.iter().sum::<u128>() / user_ratings.len() as u128)
            }
        }

    // Freelance Rating


        // Create a new freelancer rating entry
        pub fn create_freelancer_rating(&mut self, account_id: AccountId, rating: FreelancerRating) {
        self.freelancer_ratings.insert(account_id, rating);
        }

        // Read a freelancer rating entry by account ID
        pub fn get_freelancer_rating_by_account(&self, account_id: AccountId) -> Option<FreelancerRating> {
        self.freelancer_ratings.get(&account_id).cloned()
        }

        // Get freelancer ratings for a specific user (by account_id)
        pub fn get_freelancer_ratings_by_account(&self, account_id: AccountId) -> Option<Vec<FreelancerRating>> {
        self.freelancer_ratings.get(&account_id).map(|rating| vec![rating.clone()])
        }

        // Update a freelancer rating entry by account ID
        pub fn update_freelancer_rating_by_account(&mut self, account_id: AccountId, updated_rating: FreelancerRating) {
        self.freelancer_ratings.insert(account_id, updated_rating);
        }

        // Delete a freelancer rating entry by account ID
        pub fn delete_freelancer_rating_by_account(&mut self, account_id: AccountId) {
        self.freelancer_ratings.remove(&account_id);
        }

        // Calculate and return the average rating for completed projects for a specific user
        pub fn get_average_rating_for_completed_projects_by_account(&self, account_id: AccountId) -> Option<u128> {
        let user_ratings: Vec<u128> = self
            .freelancer_ratings
            .get(&account_id)
            .map(|rating| vec![rating.rating_for_completed_projects])
            .unwrap_or_else(|| Vec::new());

        if user_ratings.is_empty() {
            None
        } else {
            Some(user_ratings.iter().sum::<u128>() / user_ratings.len() as u128)
        }
        }

        // Calculate and return the average rating for communication skills for a specific user
        pub fn get_average_rating_for_communication_skills_by_account(&self, account_id: AccountId) -> Option<u128> {
            let user_ratings: Vec<u128> = self
                .freelancer_ratings
                .values()
                .filter(|rating| rating.account_id == account_id)
                .map(|rating| rating.rating_for_communication_skills)
                .collect();

            if user_ratings.is_empty() {
                None
            } else {
                Some(user_ratings.iter().sum::<u128>() / user_ratings.len() as u128)
            }
        }




        // Calculate and return the average rating for completed projects
        pub fn get_average_rating_for_completed_projects(&self) -> Option<u128> {
            let ratings: Vec<u128> = self
                .freelancer_ratings
                .values()
                .map(|rating| rating.rating_for_completed_projects)
                .collect();
            if ratings.is_empty() {
                None
            } else {
                Some(ratings.iter().sum::<u128>() / ratings.len() as u128)
            }
        }

        // Calculate and return the average rating for communication skills
        pub fn get_average_rating_for_communication_skills(&self) -> Option<u128> {
            let ratings: Vec<u128> = self
                .freelancer_ratings
                .values()
                .map(|rating| rating.rating_for_communication_skills)
                .collect();
            if ratings.is_empty() {
                None
            } else {
                Some(ratings.iter().sum::<u128>() / ratings.len() as u128)
            }
        }

    // Freelancer Bids


        // Get freelancer bids based on job ID
        pub fn get_freelancer_bids_by_job(&self, job_id: u128) -> Vec<FreelancerBid> {
            let mut bids: Vec<FreelancerBid> = self
                .freelancer_bids
                .values()
                .filter(|bid| bid.job_id == job_id)
                .cloned()
                .collect();

            // Sort the bids by bid_id
            bids.sort_by_key(|bid| bid.bid_id);

            bids
        }

        // Create a new freelancer bid for a job
        pub fn create_freelancer_bid(&mut self, job_id: u128, bid_id: u128, account_id: AccountId, bid: FreelancerBid) -> bool {
        if let Some(client_job) = self.client_jobs.get(&job_id) {
            if client_job.bid_available && bid.budget <= client_job.project_budget {
                self.freelancer_bids.insert(bid_id, bid);
                return true; // Bid created successfully
            }
        }
        false // Bid creation failed due to job not being bid available or exceeding the project budget
        }

        // Read a freelancer bid entry
        pub fn get_freelancer_bid(&self, bid_id: u128) -> Option<FreelancerBid> {
        self.freelancer_bids.get(&bid_id).cloned()
        }

        pub fn update_freelancer_bid(&mut self, bid_id: u128, updated_bid: FreelancerBid) -> bool {
        if let Some(existing_bid) = self.freelancer_bids.get(&bid_id) {
            if let Some(client_job) = self.client_jobs.get(&existing_bid.job_id) {
                if updated_bid.budget <= client_job.project_budget {
                    self.freelancer_bids.insert(bid_id, updated_bid);
                    return true; // Bid updated successfully
                }
            }
        }
        false // Bid update failed due to exceeding the project budget
        }

        // Delete a freelancer bid entry
        pub fn delete_freelancer_bid(&mut self, bid_id: u128) {
        self.freelancer_bids.remove(&bid_id);
        }


        // Accept a freelancer bid for a job
        pub fn accept_freelancer_bid(&mut self, job_id: u128, bid_id: u128) -> bool {
        let mut client_job = match self.client_jobs.get(&job_id) {
            Some(job) => job.clone(),
            None => return false, // Job does not exist
        };

        let mut freelancer_bid = match self.freelancer_bids.get_mut(&bid_id) {
            Some(bid) => bid.clone(),
            None => return false, // Bid does not exist
        };

        if !client_job.bid_available || freelancer_bid.budget > client_job.project_budget {
            return false; // Bid not eligible for acceptance
        }

        // Mark the bid as approved and set it as the accepted bid for the job
        freelancer_bid.bid_approved = true;
        client_job.bid_available = false; // Mark the job as no longer available for bids

        // Update the freelancer bid and client job in the contract's storage
        self.freelancer_bids.insert(bid_id, freelancer_bid);
        self.client_jobs.insert(job_id, client_job);

        true // Bid accepted successfully
        }

    
    // Project Milestones


        // Create a new project milestone for a bid
        pub fn create_project_milestone(
        &mut self,
        bid_id: u128,
        milestone_id: u128,
        milestone: ProjectMilestone,
        ) -> bool {
        // Check if the bid exists and is approved
        if let Some(bid) = self.freelancer_bids.get(&bid_id) {
            if !bid.bid_approved {
                return false; // Bid is not approved
            }

            // Check if the milestone budget is within the bid budget
            if milestone.milestone_budget <= bid.budget {
                // Insert the milestone into the project milestones
                self.project_milestones.insert(milestone_id, milestone);
                return true; // Milestone created successfully
            }
        }

        false // Bid does not exist, is not approved, or milestone budget exceeds bid budget
        }


        // Read a project milestone entry by milestone ID
        pub fn get_project_milestone(&self, milestone_id: u128) -> Option<ProjectMilestone> {
        self.project_milestones.get(&milestone_id).cloned()
        }

        // Get project milestones based on job ID
        pub fn get_project_milestones_by_job(&self, job_id: u128) -> Vec<ProjectMilestone> {
            let mut milestones: Vec<ProjectMilestone> = self
                .project_milestones
                .values()
                .filter(|milestone| milestone.job_id == job_id)
                .cloned()
                .collect();

            // Sort the milestones by milestone_id (you can use any key you prefer)
            milestones.sort_by_key(|milestone| milestone.milestone_id);

            milestones
        }

        // Delete a project milestone entry by milestone ID
        pub fn delete_project_milestone(&mut self, milestone_id: u128) {
        self.project_milestones.remove(&milestone_id);
        }



        // Payable function to approve a milestone
        #[payable]
        pub fn approve_milestone(&mut self, milestone_id: u128) {
            // Get the sender's account ID
            let sender_account_id = env::predecessor_account_id();

            // Check if the milestone exists
            if let Some(mut milestone) = self.project_milestones.get(&milestone_id).cloned() {
                // Get the job associated with the milestone
                if let Some(job) = self.client_jobs.get(&milestone.job_id) {
                    // Verify if the sender is the job owner
                    if job.account_id == sender_account_id {
                        // Check if the milestone is not already approved
                        if !milestone.milestone_work_approved {
                            // Check if there are unresolved disputes for the job
                            let unresolved_disputes = self.get_disputes_by_job_id(milestone.job_id);
                            if unresolved_disputes.iter().any(|dispute| !dispute.resolved) {
                                // An unresolved dispute exists; milestone approval is not allowed
                                env::panic(b"Cannot approve milestone when an unresolved dispute exists");
                            }
                            // Get the bid associated with the milestone
                            if let Some(bid) = self.freelancer_bids.get(&milestone.bid_id) {
                                // Calculate the total amount to be transferred
                                let total_amount = milestone.milestone_budget;

                                // Transfer the total amount to the freelancer's account ID
                                Promise::new(bid.account_id.clone()).transfer(total_amount);

                                // Mark the milestone as approved
                                milestone.milestone_work_approved = true;

                                // Update the milestone in the storage
                                self.project_milestones.insert(milestone_id, milestone.clone());
                            }
                        }
                    }
                }
            }
        }

}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::Balance;
    use near_sdk::test_utils::accounts;

    const NEAR: u128 = 1000000000000000000000000;


    // Auxiliar fn: create a mock context
    fn set_context(predecessor: AccountId, amount: Balance) {
      let mut builder = VMContextBuilder::new();
      builder.predecessor_account_id(predecessor);
      builder.attached_deposit(amount);

      testing_env!(builder.build());
    }

    // Freelancer Tests

    #[test]
    fn test_create_freelancer() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new freelancer
        let freelancer = Freelancer {
            account_id: user.clone(),
            profile_image: "image1".to_string(),
            full_name: "Freelancer One".to_string(),
            hourly_rate: 100,
            profession: "Designer".to_string(),
            payment_preference: "PayPal".to_string(),
            skills: vec!["Design".to_string()],
            profile_rating: 4,
            is_profile_public: true,
        };

        // Call the create_freelancer function
        contract.create_freelancer(freelancer.account_id.clone(), freelancer.clone());

        // Retrieve the created freelancer
        let retrieved_freelancer = contract.get_freelancer(freelancer.account_id.clone());

        // Assert that the retrieved freelancer matches the expected value
        assert_eq!(retrieved_freelancer, Some(freelancer));
    }

    #[test]
    fn test_update_freelancer() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new freelancer
        let mut freelancer = Freelancer {
            account_id: user.clone(),
            profile_image: "image1".to_string(),
            full_name: "Freelancer One".to_string(),
            hourly_rate: 100,
            profession: "Designer".to_string(),
            payment_preference: "PayPal".to_string(),
            skills: vec!["Design".to_string()],
            profile_rating: 4,
            is_profile_public: true,
        };

        // Call the create_freelancer function
        contract.create_freelancer(freelancer.account_id.clone(), freelancer.clone());

        // Update the freelancer's hourly rate
        freelancer.hourly_rate = 120;

        // Call the update_freelancer function
        contract.update_freelancer(freelancer.account_id.clone(), freelancer.clone());

        // Retrieve the updated freelancer
        let retrieved_freelancer = contract.get_freelancer(freelancer.account_id.clone());

        // Assert that the retrieved freelancer matches the updated value
        assert_eq!(retrieved_freelancer, Some(freelancer));
    }

    #[test]
    fn test_delete_freelancer() {
        // Initialize the contract
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new freelancer
        let freelancer = Freelancer {
            account_id: user.clone(),
            profile_image: "image1".to_string(),
            full_name: "Freelancer One".to_string(),
            hourly_rate: 100,
            profession: "Designer".to_string(),
            payment_preference: "PayPal".to_string(),
            skills: vec!["Design".to_string()],
            profile_rating: 4,
            is_profile_public: true,
        };

        // Call the create_freelancer function
        contract.create_freelancer(freelancer.account_id.clone(), freelancer.clone());

        // Call the delete_freelancer function
        contract.delete_freelancer(freelancer.account_id.clone());

        // Retrieve the deleted freelancer
        let retrieved_freelancer = contract.get_freelancer(freelancer.account_id.clone());

        // Assert that the retrieved freelancer is None (deleted)
        assert_eq!(retrieved_freelancer, None);
    }

    #[test]
    fn test_get_total_freelancers() {
        // Initialize the contract
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();
        let user2 = accounts(2);

        // Create some freelancers
        let freelancer1 = Freelancer {
            account_id: user.clone(),
            profile_image: "image1".to_string(),
            full_name: "Freelancer One".to_string(),
            hourly_rate: 100,
            profession: "Designer".to_string(),
            payment_preference: "PayPal".to_string(),
            skills: vec!["Design".to_string()],
            profile_rating: 4,
            is_profile_public: true,
        };
        let freelancer2 = Freelancer {
            account_id: user2.clone(),
            profile_image: "image2".to_string(),
            full_name: "Freelancer Two".to_string(),
            hourly_rate: 150,
            profession: "Developer".to_string(),
            payment_preference: "Crypto".to_string(),
            skills: vec!["Programming".to_string()],
            profile_rating: 5,
            is_profile_public: true,
        };

        contract.create_freelancer(freelancer1.account_id.clone(), freelancer1);
        contract.create_freelancer(freelancer2.account_id.clone(), freelancer2);

        // Call the get_total_freelancers function
        let total_freelancers = contract.get_total_freelancers();

        //user Assert that the total freelancers count matches the expected value
        assert_eq!(total_freelancers, 2);
    }

    // Freelancer Portolio Tests

    #[test]
    fn test_create_freelancer_portfolio() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new freelancer portfolio
        let portfolio = FreelancerPortfolio {
            account_id: user.clone(),
            images: vec!["image1".to_string()],
            videos: vec!["video1".to_string()],
            task_url: "task1".to_string(),
            description: "Portfolio for Freelancer One".to_string(),
        };

        // Call the create_freelancer_portfolio function
        contract.create_freelancer_portfolio(portfolio.account_id.clone(), portfolio.clone());

        // Retrieve the created freelancer portfolio
        let retrieved_portfolio = contract.get_freelancer_portfolio(portfolio.account_id.clone());

        // Assert that the retrieved portfolio matches the expected value
        assert_eq!(retrieved_portfolio, Some(portfolio));
    }

    #[test]
    fn test_update_freelancer_portfolio() {

        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new freelancer portfolio
        let mut portfolio = FreelancerPortfolio {
            account_id: user.clone(),
            images: vec!["image1".to_string()],
            videos: vec!["video1".to_string()],
            task_url: "task1".to_string(),
            description: "Portfolio for Freelancer One".to_string(),
        };

        // Call the create_freelancer_portfolio function
        contract.create_freelancer_portfolio(portfolio.account_id.clone(), portfolio.clone());

        // Update the freelancer portfolio's description
        portfolio.description = "Updated Portfolio for Freelancer One".to_string();

        // Call the update_freelancer_portfolio function
        contract.update_freelancer_portfolio(portfolio.account_id.clone(), portfolio.clone());

        // Retrieve the updated freelancer portfolio
        let retrieved_portfolio = contract.get_freelancer_portfolio(portfolio.account_id.clone());

        // Assert that the retrieved portfolio matches the updated value
        assert_eq!(retrieved_portfolio, Some(portfolio));
    }

    #[test]
    fn test_delete_freelancer_portfolio() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new freelancer portfolio
        let portfolio = FreelancerPortfolio {
            account_id: user.clone(),
            images: vec!["image1".to_string()],
            videos: vec!["video1".to_string()],
            task_url: "task1".to_string(),
            description: "Portfolio for Freelancer One".to_string(),
        };

        // Call the create_freelancer_portfolio function
        contract.create_freelancer_portfolio(portfolio.account_id.clone(), portfolio.clone());

        // Call the delete_freelancer_portfolio function
        contract.delete_freelancer_portfolio(portfolio.account_id.clone());

        // Retrieve the deleted freelancer portfolio
        let retrieved_portfolio = contract.get_freelancer_portfolio(portfolio.account_id.clone());

        // Assert that the retrieved portfolio is None (deleted)
        assert_eq!(retrieved_portfolio, None);
    }

    // Freelancer Experience Tests

    #[test]
    fn test_create_freelancer_experience() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new freelancer experience
        let experience = FreelancerExperience {
            account_id: user.clone(),
            from_date: "2021-01-01".to_string(),
            to_date: "2021-12-31".to_string(),
            job_title: "Software Engineer".to_string(),
            job_description: "Worked on project X".to_string(),
        };

        // Call the create_freelancer_experience function
        contract.create_freelancer_experience(user.clone(), experience.clone());

        // Retrieve the created freelancer experience
        let retrieved_experience = contract.get_freelancer_experience(user.clone());

        // Assert that the retrieved experience matches the expected value
        assert_eq!(retrieved_experience, Some(experience));
    }

    #[test]
    fn test_update_freelancer_experience() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new freelancer experience
        let mut experience = FreelancerExperience {
            account_id: user.clone(),
            from_date: "2021-01-01".to_string(),
            to_date: "2021-12-31".to_string(),
            job_title: "Software Engineer".to_string(),
            job_description: "Worked on project X".to_string(),
        };

        // Call the create_freelancer_experience function
        contract.create_freelancer_experience(user.clone(), experience.clone());

        // Update the freelancer experience
        experience.from_date = "2022-01-01".to_string();

        // Call the update_freelancer_experience function
        contract.update_freelancer_experience(user.clone(), experience.clone());

        // Retrieve the updated freelancer experience
        let retrieved_experience = contract.get_freelancer_experience(user.clone());

        // Assert that the retrieved experience matches the updated value
        assert_eq!(retrieved_experience, Some(experience));
    }

    #[test]
    fn test_delete_freelancer_experience() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new freelancer experience
        let experience = FreelancerExperience {
            account_id: user.clone(),
            from_date: "2021-01-01".to_string(),
            to_date: "2021-12-31".to_string(),
            job_title: "Software Engineer".to_string(),
            job_description: "Worked on project X".to_string(),
        };

        // Call the create_freelancer_experience function
        contract.create_freelancer_experience(user.clone(), experience.clone());

        // Call the delete_freelancer_experience function
        contract.delete_freelancer_experience(user.clone());

        // Retrieve the deleted freelancer experience
        let retrieved_experience = contract.get_freelancer_experience(user.clone());

        // Assert that the retrieved experience is None (deleted)
        assert_eq!(retrieved_experience, None);
    }

   // Disputes Tests
  
    #[test]
    fn test_create_dispute() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
     

        // Create a new dispute
        let dispute_id = 1;
        let dispute = Dispute {
            dispute_in_job_id: 1,
            dispute_id: dispute_id,
            dispute_name: "Dispute 1".to_string(),
            description: "Description of Dispute 1".to_string(),
            disputor: "Alice".to_string(),
            client_involved: "Bob".to_string(),
            resolved: false,
        };

        // Call the create_dispute function
        contract.create_dispute(dispute_id, dispute.clone());

        // Retrieve the created dispute
        let retrieved_dispute = contract.get_dispute(dispute_id);

        // Assert that the retrieved dispute matches the expected value
        assert_eq!(retrieved_dispute, Some(dispute));
    }

    #[test]
    fn test_update_dispute() {
        // Initialize the contract and context
        let mut contract = KaziNear::default();
   

        // Create a new dispute
        let dispute_id = 1;
        let mut dispute = Dispute {
            dispute_in_job_id: 1,
            dispute_id: dispute_id,
            dispute_name: "Dispute 1".to_string(),
            description: "Description of Dispute 1".to_string(),
            disputor: "Alice".to_string(),
            client_involved: "Bob".to_string(),
            resolved: false,
        };

        // Call the create_dispute function
        contract.create_dispute(dispute_id, dispute.clone());

        // Update the dispute
        dispute.description = "Updated Description of Dispute 1".to_string();

        // Call the update_dispute function
        contract.update_dispute(dispute_id, dispute.clone());

        // Retrieve the updated dispute
        let retrieved_dispute = contract.get_dispute(dispute_id);

        // Assert that the retrieved dispute matches the updated value
        assert_eq!(retrieved_dispute, Some(dispute));
    }

    #[test]
    fn test_delete_dispute() {
        // Initialize the contract and context
        // Initialize the contract 
        let mut contract = KaziNear::default();
      

        // Create a new dispute
        let dispute_id = 1;
        let dispute = Dispute {
            dispute_in_job_id: 1,
            dispute_id: dispute_id,
            dispute_name: "Dispute 1".to_string(),
            description: "Description of Dispute 1".to_string(),
            disputor: "Alice".to_string(),
            client_involved: "Bob".to_string(),
            resolved: false,
        };

        // Call the create_dispute function
        contract.create_dispute(dispute_id, dispute.clone());

        // Call the delete_dispute function
        contract.delete_dispute(dispute_id);

        // Retrieve the deleted dispute
        let retrieved_dispute = contract.get_dispute(dispute_id);

        // Assert that the retrieved dispute is None (deleted)
        assert_eq!(retrieved_dispute, None);
    }

    #[test]
    fn test_get_disputes_by_job_id() {
        // Initialize the contract and context
        // Initialize the contract 
        let mut contract = KaziNear::default();

        // Create a few disputes with different dispute_in_job_id values
        let dispute1 = Dispute {
            dispute_in_job_id: 1,
            dispute_id: 1,
            dispute_name: "Dispute 1".to_string(),
            description: "Description of Dispute 1".to_string(),
            disputor: "Alice".to_string(),
            client_involved: "Bob".to_string(),
            resolved: false,
        };

        let dispute2 = Dispute {
            dispute_in_job_id: 2,
            dispute_id: 2,
            dispute_name: "Dispute 2".to_string(),
            description: "Description of Dispute 2".to_string(),
            disputor: "Alice".to_string(),
            client_involved: "Bob".to_string(),
            resolved: false,
        };

        let dispute3 = Dispute {
            dispute_in_job_id: 1,
            dispute_id: 3,
            dispute_name: "Dispute 3".to_string(),
            description: "Description of Dispute 3".to_string(),
            disputor: "Alice".to_string(),
            client_involved: "Bob".to_string(),
            resolved: false,
        };

        // Call the create_dispute function for each dispute
        contract.create_dispute(dispute1.dispute_id, dispute1.clone());
        contract.create_dispute(dispute2.dispute_id, dispute2.clone());
        contract.create_dispute(dispute3.dispute_id, dispute3.clone());

        // Retrieve disputes with dispute_in_job_id = 1
        let disputes_for_job_1 = contract.get_disputes_by_job_id(1);

        // Assert that the retrieved disputes match the expected value
        assert_eq!(disputes_for_job_1.len(), 2);
        assert_eq!(disputes_for_job_1, vec![dispute1, dispute3]);
    }

    // Client Jobs
  
    #[test]
    fn test_create_client_job() {
        // Initialize the contract and context
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();
        // let initial_balance = 100_000_000_000_000; // Initial balance in yoctoNEAR
    
        // Simulate a transfer of funds to the contract
        // testing_env!(context_with_balance(user.clone(), initial_balance));


          
        // Create a new job
        let job_id = 1;
        let job = ClientJobs {
            job_id: job_id,
            account_id: user.clone(),
            project_title: "Project 1".to_string(),
            project_description: "Description of Project 1".to_string(),
            project_duration: "1 month".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill 1".to_string(), "Skill 2".to_string()],
            images: vec!["Image 1".to_string(), "Image 2".to_string()],
            bid_available: true,
        };

        let budget:Balance = job.project_budget*NEAR;


        set_context(user.clone(),budget);
    
        // Call the create_client_job function
        contract.create_client_job(job_id, job.clone());
    
        // Retrieve the created job
        let retrieved_job = contract.get_client_job(job_id);
    
        // Assert that the retrieved job matches the expected value
        assert_eq!(retrieved_job, Some(job));
    }
    

    #[test]
    fn test_update_client_job() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new job
        let job_id = 1;
        let mut job = ClientJobs {
            job_id: job_id,
            account_id: user.clone(),
            project_title: "Project 1".to_string(),
            project_description: "Description of Project 1".to_string(),
            project_duration: "1 month".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill 1".to_string(), "Skill 2".to_string()],
            images: vec!["Image 1".to_string(), "Image 2".to_string()],
            bid_available: true,
        };


        let budget:Balance = job.project_budget*NEAR;

        set_context(user.clone(),budget);
        // Call the create_client_job function
        contract.create_client_job(job_id, job.clone());

        // Update the job
        job.project_description = "Updated Description of Project 1".to_string();

        // Call the update_client_job function
        contract.update_client_job(job_id, job.clone());

        // Retrieve the updated job
        let retrieved_job = contract.get_client_job(job_id);

        // Assert that the retrieved job matches the updated value
        assert_eq!(retrieved_job, Some(job));
    }

    #[test]
    fn test_delete_client_job() {
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();

        // Create a new job
        let job_id = 1;
        let job = ClientJobs {
            job_id: job_id,
            account_id: user.clone(),
            project_title: "Project 1".to_string(),
            project_description: "Description of Project 1".to_string(),
            project_duration: "1 month".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill 1".to_string(), "Skill 2".to_string()],
            images: vec!["Image 1".to_string(), "Image 2".to_string()],
            bid_available: true,
        };


        let budget:Balance = job.project_budget*NEAR;
        
        set_context(user.clone(),budget);
    
        // Call the create_client_job function
        contract.create_client_job(job_id, job.clone());

        // Call the delete_client_job function
        contract.delete_client_job(job_id);

        // Retrieve the deleted job
        let retrieved_job = contract.get_client_job(job_id);
        
        // Assert that the retrieved job is None (deleted)
        assert_eq!(retrieved_job, None);
    }

    #[test]
    fn test_get_all_jobs_for_account() {
        // Initialize the contract and context
        // Initialize the contract 
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();
        let user2 = accounts(2);


        // Create jobs for different accounts
        let job1 = ClientJobs {
            job_id: 1,
            account_id: user.clone(),
            project_title: "Project 1".to_string(),
            project_description: "Description of Project 1".to_string(),
            project_duration: "1 month".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill 1".to_string(), "Skill 2".to_string()],
            images: vec!["Image 1".to_string(), "Image 2".to_string()],
            bid_available: true,
        };

        let job2 = ClientJobs {
            job_id: 2,
            account_id: user2.clone(),
            project_title: "Project 2".to_string(),
            project_description: "Description of Project 2".to_string(),
            project_duration: "2 months".to_string(),
            project_budget: 2000,
            skill_requirements: vec!["Skill 3".to_string()],
            images: vec!["Image 3".to_string()],
            bid_available: false,
        };
        let budget1:Balance = job1.project_budget*NEAR;

        set_context(user.clone(),budget1);
    
        // Call the create_client_job function for each job
        contract.create_client_job(job1.job_id, job1.clone());

        let budget2:Balance = job2.project_budget*NEAR;

        set_context(user.clone(),budget2);
        contract.create_client_job(job2.job_id, job2.clone());

        // Retrieve all jobs for account "alice"
        let jobs_for_alice = contract.get_all_jobs_for_account(user.clone());

        // Assert that the retrieved jobs match the expected value
        assert_eq!(jobs_for_alice.len(), 1);
        assert_eq!(jobs_for_alice, vec![job1]);
    }

    #[test]
    fn test_get_all_client_jobs() {

      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        // Create multiple jobs
        let job1 = ClientJobs {
            job_id: 1,
            account_id: user.clone(),
            project_title: "Project 1".to_string(),
            project_description: "Description of Project 1".to_string(),
            project_duration: "1 month".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill 1".to_string(), "Skill 2".to_string()],
            images: vec!["Image 1".to_string(), "Image 2".to_string()],
            bid_available: true,
        };

        let job2 = ClientJobs {
            job_id: 2,
            account_id: user2.clone(),
            project_title: "Project 2".to_string(),
            project_description: "Description of Project 2".to_string(),
            project_duration: "2 months".to_string(),
            project_budget: 2000,
            skill_requirements: vec!["Skill 3".to_string()],
            images: vec!["Image 3".to_string()],
            bid_available: false,
        };

        let budget1:Balance = job1.project_budget*NEAR;

        set_context(user.clone(),budget1);
        // Call the create_client_job function for each job
        contract.create_client_job(job1.job_id, job1.clone());


        let budget2:Balance = job2.project_budget*NEAR;

        set_context(user.clone(),budget2);
    
        contract.create_client_job(job2.job_id, job2.clone());

        // Retrieve all client jobs
        let all_client_jobs = contract.get_all_client_jobs();

        // Assert that the retrieved jobs match the expected value
        assert_eq!(all_client_jobs.len(), 2);
        assert_eq!(all_client_jobs, vec![job1, job2]);
    }

    // Chats Tests
  
    #[test]
    fn test_create_chat() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        // Create a new chat
        let sender_id = user.clone();
        let timestamp = 1;
        let chat = Chat {
            timestamp: timestamp,
            sender: user.clone(),
            receiver: user2.clone(),
            message: "Hello, Bob!".to_string(),
            attached_files: vec!["file1".to_string(), "file2".to_string()],
            seen: false,
        };

        // Call the create_chat function
        contract.create_chat(sender_id.clone(),chat.clone());

        // Retrieve the created chat
        let retrieved_chat = contract.get_chat(sender_id.clone());

        // Assert that the retrieved chat matches the expected value
        assert_eq!(retrieved_chat, Some(chat));
    }

    #[test]
    fn test_update_chat() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        // Create a new chat
        let sender_id = user.clone();
        let timestamp = 1;
        let mut chat = Chat {
            timestamp: timestamp,
            sender: user.clone(),
            receiver: user2.clone(),
            message: "Hello, Bob!".to_string(),
            attached_files: vec!["file1".to_string(), "file2".to_string()],
            seen: false,
        };

        // Call the create_chat function
        contract.create_chat(sender_id.clone(), chat.clone());

        // Update the chat
        chat.message = "Updated message for Bob".to_string();

        // Call the update_chat function
        contract.update_chat(sender_id.clone(), chat.clone());

        // Retrieve the updated chat
        let retrieved_chat = contract.get_chat(sender_id.clone());

        // Assert that the retrieved chat matches the updated value
        assert_eq!(retrieved_chat, Some(chat));
    }

    #[test]
    fn test_delete_chat() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        // Create a new chat
        let sender_id = user.clone();
        let timestamp = 1;
        let chat = Chat {
            timestamp: timestamp,
            sender: user.clone(),
            receiver: user2.clone(),
            message: "Hello, Bob!".to_string(),
            attached_files: vec!["file1".to_string(), "file2".to_string()],
            seen: false,
        };

        // Call the create_chat function
        contract.create_chat(sender_id.clone(),  chat.clone());

        // Call the delete_chat function
        contract.delete_chat(sender_id.clone());

        // Retrieve the deleted chat
        let retrieved_chat = contract.get_chat(sender_id.clone());

        // Assert that the retrieved chat is None (deleted)
        assert_eq!(retrieved_chat, None);
    }

    #[test]
    fn test_get_all_chats_for_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        // Create chats for different accounts
        let chat1 = Chat {
            timestamp: 1,
            sender: user.clone(),
            receiver: user2.clone(),
            message: "Hello, Bob!".to_string(),
            attached_files: vec!["file1".to_string(), "file2".to_string()],
            seen: false,
        };

        let chat2 = Chat {
            timestamp: 2,
            sender: user2.clone(),
            receiver: user.clone(),
            message: "Hi, Alice!".to_string(),
            attached_files: vec!["file3".to_string()],
            seen: true,
        };

        // Call the create_chat function for each chat
        contract.create_chat(chat1.sender.clone(),  chat1.clone());
        contract.create_chat(chat2.sender.clone(), chat2.clone());

        // Retrieve all chats for account "alice"
        let chats_for_alice = contract.get_all_chats_for_account(user);

        // Assert that the retrieved chats match the expected value
        assert_eq!(chats_for_alice.len(), 2);
        assert_eq!(chats_for_alice, vec![chat1,chat2]);
    }


    // Client Rating

    #[test]
    fn test_create_client_rating() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating = ClientRatings {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_timely_payments: 5,
            rating_for_timely_feedbacks: 4,
        };

        contract.create_client_rating(user2.clone(), rating.clone());

        let retrieved_rating = contract.get_client_rating_by_account(user2.clone()).unwrap();
        assert_eq!(retrieved_rating, rating);
    }

    #[test]
    fn test_get_client_ratings_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating = ClientRatings {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_timely_payments: 5,
            rating_for_timely_feedbacks: 4,
        };

        contract.create_client_rating(user2.clone(),rating.clone());

        let retrieved_ratings = contract.get_client_ratings_by_account(user2.clone()).unwrap();
        assert_eq!(retrieved_ratings.len(), 1);
        assert_eq!(retrieved_ratings[0], rating);
    }

    #[test]
    fn test_update_client_rating_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating1 = ClientRatings {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_timely_payments: 5,
            rating_for_timely_feedbacks: 4,
        };

        contract.create_client_rating(user2.clone(),rating1.clone());

        let updated_rating = ClientRatings {
            rating_id: 1,
            project_name: "Updated Project".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Updated feedback".to_string(),
            rating_for_timely_payments: 3,
            rating_for_timely_feedbacks: 5,
        };

        contract.update_client_rating_by_account(user2.clone(), updated_rating.clone());

        let retrieved_rating = contract.get_client_rating_by_account(user2).unwrap();
        assert_eq!(retrieved_rating, updated_rating);
    }

    #[test]
    fn test_delete_client_rating_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating = ClientRatings {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_timely_payments: 5,
            rating_for_timely_feedbacks: 4,
        };

        contract.create_client_rating(user2.clone(),rating.clone());

        contract.delete_client_rating_by_account(user2.clone());

        let retrieved_rating = contract.get_client_rating_by_account(user2.clone());
        assert!(retrieved_rating.is_none());
    }

    #[test]
    fn test_get_average_rating_for_timely_payments_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating1 = ClientRatings {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user.clone(),
            feedback: "Good job".to_string(),
            rating_for_timely_payments: 5,
            rating_for_timely_feedbacks: 4,
        };

        let rating2 = ClientRatings {
            rating_id: 2,
            project_name: "Project2".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Excellent job".to_string(),
            rating_for_timely_payments: 4,
            rating_for_timely_feedbacks: 5,
        };

        contract.create_client_rating(user.clone(), rating1.clone());
        contract.create_client_rating(user2, rating2.clone());

        let average = contract.get_average_rating_for_timely_payments_by_account(user).unwrap();
        assert_eq!(average, 4); // (5 + 4) / 2 = 4.5, but rounded down to 4
    }

    #[test]
    fn test_get_average_rating_for_timely_feedbacks_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating1 = ClientRatings {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user.clone(),
            feedback: "Good job".to_string(),
            rating_for_timely_payments: 5,
            rating_for_timely_feedbacks: 4,
        };

        let rating2 = ClientRatings {
            rating_id: 2,
            project_name: "Project2".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Excellent job".to_string(),
            rating_for_timely_payments: 4,
            rating_for_timely_feedbacks: 5,
        };

        contract.create_client_rating(user.clone(),  rating1.clone());
        contract.create_client_rating(user2.clone(),  rating2.clone());

        let average = contract.get_average_rating_for_timely_feedbacks_by_account(user).unwrap();
        assert_eq!(average, 4); // (4 + 5) / 2 = 4.5, but rounded down to 4
    }


    #[test]
    fn test_get_client_rating_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating = ClientRatings {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_timely_payments: 5,
            rating_for_timely_feedbacks: 4,
        };

        contract.create_client_rating(user2.clone(), rating.clone());

        let retrieved_rating = contract.get_client_rating_by_account(user2).unwrap();
        assert_eq!(retrieved_rating, rating);
    }

    // Freelancer Ratings Tests

    #[test]
    fn test_create_freelancer_rating() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      

      let user2= accounts(2);


        let rating = FreelancerRating {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_completed_projects: 5,
            rating_for_communication_skills: 4,
        };

        contract.create_freelancer_rating(user2.clone(), rating.clone());

        let retrieved_rating = contract.get_freelancer_rating_by_account(user2).unwrap();
        assert_eq!(retrieved_rating, rating);
    }

    #[test]
    fn test_get_freelancer_ratings_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating = FreelancerRating {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_completed_projects: 5,
            rating_for_communication_skills: 4,
        };

        contract.create_freelancer_rating(user2.clone(), rating.clone());

        let retrieved_ratings = contract.get_freelancer_ratings_by_account(user2).unwrap();
        assert_eq!(retrieved_ratings.len(), 1);
        assert_eq!(retrieved_ratings[0], rating);
    }

    #[test]
    fn test_update_freelancer_rating_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating1 = FreelancerRating {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_completed_projects: 5,
            rating_for_communication_skills: 4,
        };

        contract.create_freelancer_rating(user2.clone(), rating1.clone());

        let updated_rating = FreelancerRating {
            rating_id: 1,
            project_name: "Updated Project".to_string(),
            account_id: user.clone(),
            rating_user: user.clone(),
            feedback: "Updated feedback".to_string(),
            rating_for_completed_projects: 3,
            rating_for_communication_skills: 5,
        };

        contract.update_freelancer_rating_by_account(user.clone(), updated_rating.clone());

        let retrieved_rating = contract.get_freelancer_rating_by_account(user).unwrap();
        assert_eq!(retrieved_rating, updated_rating);
    }

    #[test]
    fn test_delete_freelancer_rating_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating = FreelancerRating {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_completed_projects: 5,
            rating_for_communication_skills: 4,
        };

        contract.create_freelancer_rating(user2.clone(), rating.clone());

        contract.delete_freelancer_rating_by_account(user2.clone());

        let retrieved_rating = contract.get_freelancer_rating_by_account(user2.clone());
        assert!(retrieved_rating.is_none());
    }

    #[test]
    fn test_get_average_rating_for_completed_projects_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating1 = FreelancerRating {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_completed_projects: 5,
            rating_for_communication_skills: 4,
        };

        let rating2 = FreelancerRating {
            rating_id: 2,
            project_name: "Project2".to_string(),
            account_id: user.clone(),
            rating_user: user.clone(),
            feedback: "Excellent job".to_string(),
            rating_for_completed_projects: 4,
            rating_for_communication_skills: 5,
        };

        contract.create_freelancer_rating(user2.clone(),  rating1.clone());
        contract.create_freelancer_rating(user.clone(), rating2.clone());

        let average = contract.get_average_rating_for_completed_projects_by_account(user).unwrap();
        assert_eq!(average, 4); // (5 + 4) / 2 = 4.5, but rounded down to 4
    }

    #[test]
    fn test_get_average_rating_for_communication_skills_by_account() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let rating1 = FreelancerRating {
            rating_id: 1,
            project_name: "Project1".to_string(),
            account_id: user.clone(),
            rating_user: user2.clone(),
            feedback: "Good job".to_string(),
            rating_for_completed_projects: 5,
            rating_for_communication_skills: 4,
        };

        let rating2 = FreelancerRating {
            rating_id: 2,
            project_name: "Project2".to_string(),
            account_id: user.clone(),
            rating_user: user.clone(),
            feedback: "Excellent job".to_string(),
            rating_for_completed_projects: 2,
            rating_for_communication_skills: 2,
        };

        contract.create_freelancer_rating(user2.clone(), rating1.clone());
        contract.create_freelancer_rating(user.clone(), rating2.clone());

        let average = contract.get_average_rating_for_communication_skills_by_account(user).unwrap();
        assert_eq!(average, 3); // (4 + 5) / 2 = 4.5, but rounded down to 4
    }



    #[test]
    fn test_create_freelancer_bid_valid() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let job_id = 1;
        let bid_id = 1;
        let client_job = ClientJobs {
            job_id,
            account_id: user.clone(),
            project_title: "Project1".to_string(),
            project_description: "Description1".to_string(),
            project_duration: "30 days".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill1".to_string()],
            images: vec!["Image1".to_string()],
            bid_available: true,
        };

        let budget:Balance = client_job.project_budget*NEAR;

        set_context(user.clone(),budget);

        contract.create_client_job(job_id, client_job.clone());

        let freelancer_bid = FreelancerBid {
            job_id,
            bid_id,
            account_id: user2.clone(),
            bid_description: "Bid description".to_string(),
            budget: 500,
            relevant_files: vec!["File1".to_string()],
            bid_approved: false,
        };
        let result = contract.create_freelancer_bid(job_id, bid_id, user, freelancer_bid.clone());

        assert_eq!(result, true);

        let result_bid = contract.get_freelancer_bid(bid_id);
        assert_eq!(result_bid, Some(freelancer_bid));
    }

    #[test]
    fn test_create_freelancer_bid_invalid_job_not_available() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let job_id = 1;
        let bid_id = 1;
        let client_job = ClientJobs {
            job_id,
            account_id: user.clone(),
            project_title: "Project1".to_string(),
            project_description: "Description1".to_string(),
            project_duration: "30 days".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill1".to_string()],
            images: vec!["Image1".to_string()],
            bid_available: false, // Job is not available for bids
        };

        let budget:Balance = client_job.project_budget*NEAR;

        set_context(user.clone(),budget);

        contract.create_client_job(job_id, client_job.clone());

        let freelancer_bid = FreelancerBid {
            job_id: 1,
            bid_id,
            account_id: user2.clone(),
            bid_description: "Bid description".to_string(),
            budget: 500,
            relevant_files: vec!["File1".to_string()],
            bid_approved: false,
        };
        let result = contract.create_freelancer_bid(job_id, bid_id, user2, freelancer_bid.clone());

        assert_eq!(result, false);

        let result_bid = contract.get_freelancer_bid(bid_id);
        assert_eq!(result_bid, None);
    }

    // Add tests for other CRUD operations and validations as needed

    #[test]
    fn test_get_freelancer_bids_by_job() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);

      let user3= accounts(3);


        let job_id = 1;
        let bid_id1 = 1;
        let bid_id2 = 2;

        let client_job = ClientJobs {
            job_id,
            account_id: user.clone(),
            project_title: "Project1".to_string(),
            project_description: "Description1".to_string(),
            project_duration: "30 days".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill1".to_string()],
            images: vec!["Image1".to_string()],
            bid_available: true,
        };

        let budget:Balance = client_job.project_budget*NEAR;

        set_context(user.clone(),budget);

        contract.create_client_job(job_id, client_job.clone());

        let freelancer_bid1 = FreelancerBid {
            job_id,
            bid_id: bid_id1,
            account_id: user2.clone(),
            bid_description: "Bid description 1".to_string(),
            budget: 500,
            relevant_files: vec!["File1".to_string()],
            bid_approved: false,
        };

        let freelancer_bid2 = FreelancerBid {
            job_id,
            bid_id: bid_id2,
            account_id: user3.clone(),
            bid_description: "Bid description 2".to_string(),
            budget: 800,
            relevant_files: vec!["File2".to_string()],
            bid_approved: false,
        };

        contract.create_freelancer_bid(job_id, bid_id1, user2, freelancer_bid1.clone());
        contract.create_freelancer_bid(job_id, bid_id2, user3, freelancer_bid2.clone());

        let bids_by_job = contract.get_freelancer_bids_by_job(job_id);
        assert_eq!(bids_by_job, vec![freelancer_bid1, freelancer_bid2]);
    }


    #[test]
    fn test_delete_freelancer_bid() {
        // Arrange: set up the test environment, context, and contract
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();
        let user2 = accounts(2);


        let job_id = 1;
        let bid_id = 1;

        // Create a new client job
        let client_job = ClientJobs {
            job_id,
            account_id: user.clone(),
            project_title: "Project1".to_string(),
            project_description: "Description1".to_string(),
            project_duration: "30 days".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill1".to_string()],
            images: vec!["Image1".to_string()],
            bid_available: true,
        };

        let budget:Balance = client_job.project_budget*NEAR;

        set_context(user.clone(),budget);

        contract.create_client_job(job_id, client_job);

        // Create a new freelancer bid
        let freelancer_bid = FreelancerBid {
            job_id,
            bid_id,
            account_id: user2.clone(),
            bid_description: "Bid description".to_string(),
            budget: 500,
            relevant_files: vec!["File1".to_string()],
            bid_approved: false,
        };
        contract.create_freelancer_bid(job_id, bid_id, user, freelancer_bid);

        // Act: delete the freelancer bid
        contract.delete_freelancer_bid(bid_id);

        // Assert: verify that the freelancer bid no longer exists
        let result_bid = contract.get_freelancer_bid(bid_id);
        assert_eq!(result_bid, None);
    }


    // Test accepting a freelancer bid
    #[test]
    fn test_accept_freelancer_bid() {
      let mut contract = KaziNear::default();
      let user = env::predecessor_account_id();
      let user2 = accounts(2);


        let job_id = 1;
        let bid_id = 1;

        // Create a new client job
        let client_job = ClientJobs {
            job_id,
            account_id: user.clone(),
            project_title: "Project1".to_string(),
            project_description: "Description1".to_string(),
            project_duration: "30 days".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill1".to_string()],
            images: vec!["Image1".to_string()],
            bid_available: true,
        };

        let budget:Balance = client_job.project_budget*NEAR;

        set_context(user.clone(),budget);

        contract.create_client_job(job_id, client_job);

        // Create a new freelancer bid
        let freelancer_bid = FreelancerBid {
            job_id,
            bid_id,
            account_id: user2.clone(),
            bid_description: "Bid description".to_string(),
            budget: 500,
            relevant_files: vec!["File1".to_string()],
            bid_approved: false,
        };
        contract.create_freelancer_bid(job_id, bid_id, user2.clone(), freelancer_bid);

        // Act: accept the freelancer bid
        let result = contract.accept_freelancer_bid(job_id, bid_id);

        // Assert: verify that the bid is accepted and job is no longer bid available
        assert!(result);

        let accepted_bid = contract.get_freelancer_bid(bid_id).unwrap();
        assert!(accepted_bid.bid_approved);

        let accepted_job = contract.get_client_job(job_id).unwrap();
        assert!(!accepted_job.bid_available);
    }

    #[test]
    fn test_create_project_milestone_valid_budget() {
        // Arrange: set up the test environment, context, and contract
        let mut contract = KaziNear::default();
        let user = env::predecessor_account_id();
        let user2 = accounts(2);


        let bid_id = 1;
        let milestone_id = 1;
        let job_id = 1;

        // Create a new client job
        let client_job = ClientJobs {
            job_id,
            account_id: user.clone(),
            project_title: "Project1".to_string(),
            project_description: "Description1".to_string(),
            project_duration: "30 days".to_string(),
            project_budget: 1000,
            skill_requirements: vec!["Skill1".to_string()],
            images: vec!["Image1".to_string()],
            bid_available: true,
        };

        let budget:Balance = client_job.project_budget*NEAR;

        set_context(user.clone(),budget);

        contract.create_client_job(job_id, client_job);

        // Create a freelancer bid with a budget
        let freelancer_bid = FreelancerBid {
            job_id: 1,
            bid_id: bid_id,
            account_id: user2.clone(),
            bid_description: "Bid description".to_string(),
            budget: 1000, // Freelancer's budget
            relevant_files: vec!["File1".to_string()],
            bid_approved: false, // Approved bid
        };

        contract.create_freelancer_bid(1, bid_id, user2.clone(), freelancer_bid);

        let result_approved = contract.accept_freelancer_bid(job_id, bid_id);

        assert!(result_approved);

        // Create a project milestone with a budget within the freelancer's budget
        let milestone = ProjectMilestone {
            job_id: 1,
            bid_id: bid_id,
            milestone_id: milestone_id,
            milestone_name: "Milestone 1".to_string(),
            milestone_description: "Description 1".to_string(),
            milestone_budget: 500, // Milestone budget within freelancer's budget
            milestone_duration: 7,
            milestone_work_approved: false,
        };

        // Act: create the project milestone
        let result = contract.create_project_milestone(bid_id, milestone_id, milestone);

        // Assert: verify that the milestone is created successfully
        assert!(result);

        let created_milestone = contract.get_project_milestone(milestone_id).unwrap();
        assert_eq!(created_milestone.milestone_budget, 500);
    }


        // Test getting a project milestone by milestone ID
        #[test]
        fn test_get_project_milestone() {
            // Arrange: set up the test environment, context, and contract
            let mut contract = KaziNear::default();
            let user = env::predecessor_account_id();
            let user2 = accounts(2);

            let bid_id = 1;
            let milestone_id = 1;
            let job_id = 1;
    
            // Create a new client job
            let client_job = ClientJobs {
                job_id,
                account_id: user.clone(),
                project_title: "Project1".to_string(),
                project_description: "Description1".to_string(),
                project_duration: "30 days".to_string(),
                project_budget: 1000,
                skill_requirements: vec!["Skill1".to_string()],
                images: vec!["Image1".to_string()],
                bid_available: true,
            };
    
            let budget:Balance = client_job.project_budget*NEAR;
    
            set_context(user.clone(),budget);
    
            contract.create_client_job(job_id, client_job);
    
            // Create a freelancer bid with a budget
            let freelancer_bid = FreelancerBid {
                job_id: 1,
                bid_id: bid_id,
                account_id: user2.clone(),
                bid_description: "Bid description".to_string(),
                budget: 1000, // Freelancer's budget
                relevant_files: vec!["File1".to_string()],
                bid_approved: false, // Approved bid
            };
    
            contract.create_freelancer_bid(1, bid_id, user2.clone(), freelancer_bid);
    
            let result_approved = contract.accept_freelancer_bid(job_id, bid_id);
    
            assert!(result_approved);

    
            let milestone_id = 1;
            let milestone = ProjectMilestone {
                job_id: 1,
                bid_id: 1,
                milestone_id: milestone_id,
                milestone_name: "Milestone 1".to_string(),
                milestone_description: "Description 1".to_string(),
                milestone_budget: 500,
                milestone_duration: 7,
                milestone_work_approved: false,
            };
    
            // Create a project milestone
            contract.create_project_milestone(1, milestone_id, milestone.clone());
    
            // Act: get the project milestone by milestone ID
            let retrieved_milestone = contract.get_project_milestone(milestone_id);
    
            // Assert: verify that the retrieved milestone matches the created one
            assert_eq!(retrieved_milestone, Some(milestone));
        }
    
        // Test getting project milestones by job ID
        #[test]
        fn test_get_project_milestones_by_job() {
            // Arrange: set up the test environment, context, and contract
            let mut contract = KaziNear::default();
            let user = env::predecessor_account_id();
            let user2 = accounts(2);


            let bid_id = 1;
            let milestone_id = 1;
            let job_id = 1;
    
            // Create a new client job
            let client_job = ClientJobs {
                job_id,
                account_id: user.clone(),
                project_title: "Project1".to_string(),
                project_description: "Description1".to_string(),
                project_duration: "30 days".to_string(),
                project_budget: 1000,
                skill_requirements: vec!["Skill1".to_string()],
                images: vec!["Image1".to_string()],
                bid_available: true,
            };
    
            let budget:Balance = client_job.project_budget*NEAR;
    
            set_context(user.clone(),budget);
    
            contract.create_client_job(job_id, client_job);
    
            // Create a freelancer bid with a budget
            let freelancer_bid = FreelancerBid {
                job_id: 1,
                bid_id: bid_id,
                account_id: user2.clone(),
                bid_description: "Bid description".to_string(),
                budget: 1000, // Freelancer's budget
                relevant_files: vec!["File1".to_string()],
                bid_approved: false, // Approved bid
            };
    
            contract.create_freelancer_bid(1, bid_id, user2.clone(), freelancer_bid);
    
            let result_approved = contract.accept_freelancer_bid(job_id, bid_id);
    
            assert!(result_approved);


            let job_id = 1;
            let milestone1 = ProjectMilestone {
                job_id: job_id,
                bid_id: 1,
                milestone_id: 1,
                milestone_name: "Milestone 1".to_string(),
                milestone_description: "Description 1".to_string(),
                milestone_budget: 500,
                milestone_duration: 7,
                milestone_work_approved: false,
            };
            let milestone2 = ProjectMilestone {
                job_id: job_id,
                bid_id: 1,
                milestone_id: 2,
                milestone_name: "Milestone 2".to_string(),
                milestone_description: "Description 2".to_string(),
                milestone_budget: 500,
                milestone_duration: 5,
                milestone_work_approved: false,
            };
    
            // Create project milestones
            contract.create_project_milestone(1, 1, milestone1.clone());
            contract.create_project_milestone(1, 2, milestone2.clone());
    
            // Act: get project milestones by job ID
            let milestones = contract.get_project_milestones_by_job(job_id);
    
            // Assert: verify that the retrieved milestones match the created ones
            assert_eq!(milestones, vec![milestone1, milestone2]);
        }
    
        // Test deleting a project milestone
        #[test]
        fn test_delete_project_milestone() {
            // Arrange: set up the test environment, context, and contract
            let mut contract = KaziNear::default();
            let user = env::predecessor_account_id();
            let user2 = accounts(2);


            let milestone_id = 1;
            let milestone = ProjectMilestone {
                job_id: 1,
                bid_id: 1,
                milestone_id: milestone_id,
                milestone_name: "Milestone 1".to_string(),
                milestone_description: "Description 1".to_string(),
                milestone_budget: 500,
                milestone_duration: 7,
                milestone_work_approved: false,
            };
    
            // Create a project milestone
            contract.create_project_milestone(1, milestone_id, milestone.clone());
    
            // Act: delete the project milestone
            contract.delete_project_milestone(milestone_id);
    
            // Assert: verify that the milestone is deleted
            let deleted_milestone = contract.get_project_milestone(milestone_id);
            assert_eq!(deleted_milestone, None);
        }
    

        #[test]
        fn test_accept_freelancer_bid_and_approve_milestone() {
            // Initialize the contract
            let mut contract = KaziNear::default();
            let user = env::predecessor_account_id();
            let user2 = accounts(2);
            // Create a sample job and add it to the contract
            let job_id = 1;
            let job_owner_account_id = env::predecessor_account_id(); // Replace with the job owner's account ID
            let job = ClientJobs {
                job_id: job_id,
                account_id: job_owner_account_id.clone(),
                project_title: "Project 1".to_string(),
                project_description: "Description of Project 1".to_string(),
                project_duration: "1 month".to_string(),
                project_budget: 1000, // Job budget
                skill_requirements: vec!["Skill 1".to_string(), "Skill 2".to_string()],
                images: vec!["Image 1".to_string(), "Image 2".to_string()],
                bid_available: true,
            };

            let budget:Balance = job.project_budget*NEAR;

            set_context(user.clone(),budget);
        
            // Attach the job's budget as the deposit when creating the client job
            // let attached_deposit = job.project_budget; // Deposit equals the job's budget
            contract.create_client_job(job_id, job.clone());
        
            // Create a sample freelancer bid and add it to the contract
            let bid_id = 1;
            let freelancer_account_id = env::predecessor_account_id(); // Replace with the freelancer's account ID
            let bid = FreelancerBid {
                job_id: job_id,
                bid_id: bid_id,
                account_id: freelancer_account_id.clone(),
                bid_description: "Sample bid description".to_string(),
                budget: 500, // Bid budget less than job budget
                relevant_files: vec![],
                bid_approved: false, // Bid is not yet approved
            };
            contract.create_freelancer_bid(job_id,bid_id,freelancer_account_id, bid.clone());
        
            // Attach the bid budget as the deposit when accepting the bid
             // Deposit equals the bid's budget
            let bid_accepted = contract.accept_freelancer_bid(job_id, bid_id);
            assert!(bid_accepted, "Failed to accept the freelancer bid");

                    // Create a new dispute
            let dispute_id = 1;
            let mut dispute = Dispute {
                dispute_in_job_id: 1,
                dispute_id: dispute_id,
                dispute_name: "Dispute 1".to_string(),
                description: "Description of Dispute 1".to_string(),
                disputor: "Alice".to_string(),
                client_involved: "Bob".to_string(),
                resolved: false,
            };

            // Call the create_dispute function
            contract.create_dispute(dispute_id, dispute.clone());

            // Update the dispute
            dispute.description = "Updated Description of Dispute 1".to_string();
            dispute.resolved = true;

            // Call the update_dispute function
            contract.update_dispute(dispute_id, dispute.clone());

            // Retrieve the updated dispute
            let retrieved_dispute = contract.get_dispute(dispute_id);

            // Assert that the retrieved dispute matches the updated value
            assert_eq!(retrieved_dispute, Some(dispute));
        
            // Create a sample milestone and add it to the contract
            let milestone_id = 1;
            let milestone = ProjectMilestone {
                job_id: job_id,
                bid_id: bid_id,
                milestone_id: milestone_id,
                milestone_name: "Milestone 1".to_string(),
                milestone_description: "Description of Milestone 1".to_string(),
                milestone_budget: 200, // Milestone budget
                milestone_duration: 7, // Milestone duration
                milestone_work_approved: false, // Milestone is not approved yet
            };
            contract.create_project_milestone(1,milestone_id, milestone.clone());
        
            // Attach the milestone amount as the deposit when approving the milestone
            let attached_deposit = milestone.milestone_budget; // Deposit equals the milestone's budget
            let milestone_approved = contract.approve_milestone(milestone_id);
            // assert!(milestone_approved, "Failed to approve the milestone");
        
            // Verify that the milestone is marked as approved
            let approved_milestone = contract.get_project_milestone(milestone_id);
            assert_eq!(approved_milestone.unwrap().milestone_work_approved, true);
        
            // Verify that the funds are transferred to the freelancer's account
            // You can add assertions based on your contract's logic and expectations
        }
        

}