use rusoto_core::Region;
use rusoto_ec2::{Ec2, Ec2Client};

// tmp function to test this out
pub fn main() {
  
  // describe instance request struct
  // filter for ec2 instance id
  let ec2_running_filter: Vec<rusoto_ec2::generated::Filter> = rusoto_ec2::Filter {
    name: Some("instance_state_name".to_string()),
    values: Some(vec!["running".to_string()]),
  };

  // describe instance request
  let get_all_ec2 = rusoto_ec2::DescribeInstancesRequest { 
    filters: Some(ec2_running_filter),
    dry_run: None,
    max_results: None,
    next_token: None,
    instance_ids: None,
  };


}
