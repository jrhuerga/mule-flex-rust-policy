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
Following steps describe how to download the code from GitHub and compile it.

1. Clone the repo localy in the linux box created previously:

    `$ git clone https://github.com/jrhuerga/mule-flex-rust-policy.git`

2. Change to the downloaded folder and configure the target as WebAssembly

    `$ rustup target add wasm32-unknown-unknown`

3. Compile the code using Cargo (Cargo is Rust's build system and package manager)

    `cargo build --target wasm32-unknown-unknown --release`

4. This will generate a file named flex_custom_policy_auth_header.wasm in the folder target/wasm32-unknown-unknown/release . You can optionally use wasm-gc to reduce its size. If necessary, you can use an AWS S3 bucket to copy that file so it will be available on your laptop to upload it to MuleSoft Exchange using a browser.




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


