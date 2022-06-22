# mule-flex-rust-policy
This is a demo of a Rust policy for MuleSoft Anypoint Flex. The policy essentially checks if the request contains a header x-custom-auth and if its value is the same configured in the policy. Otherwise, it returns a 401.
