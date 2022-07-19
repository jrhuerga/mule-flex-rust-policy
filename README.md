# MuleSoft Anypoint Flex Gateway policy (custom header check)
This is a Rust policy for MuleSoft Anypoint Flex. The policy essentially checks if the request contains a header x-custom-auth and if its value is the same configured in the policy. If that is the case, it allows the execution of the backend; otherwise, it returns a 401.

For more informaton check: [Implementing a Flex Gateway Custom Policy in Rust](https://docs.mulesoft.com/gateway/policies-custom-flex-implement-rust)

## Configuring a Rust development environment

Following steps describe how to configure a development environment on an EC2 linux instance:

1. Create an EC2 instance with Ubuntu, of type t2.medium
2. Download and install Rust

    `$ curl https://sh.rustup.rs -sSf | sh`

3. Install build essentials

    `$ sudo apt-get update`

    `$ sudo apt install build-essential`

4. Configure Rust

    `$ sudo snap install rustup --classic`

    `$ rustup install stable`

    `$ rustup default stable`


## Compiling the code
Pending

1. Point
1. Point
1. Point

`$ sudo rm`


## Registering a Flex Gateway
Pending

1. Point
1. Point
1. Point

`$ sudo rm`


## Publishing the policy in Exchange
Pending

1. Point
1. Point
1. Point

`$ sudo rm`


## Testing the policy
Pending

1. Point
1. Point
1. Point

`$ sudo rm`


