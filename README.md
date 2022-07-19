# MuleSoft Anypoint Flex Gateway policy (custom header check)
This is a Rust policy for MuleSoft Anypoint Flex. The policy essentially checks if the request contains a header _x-custom-auth_ and if its value is the same configured in the policy. If that is the case, it allows the execution of the backend; otherwise, it returns a 401.

For more informaton check: [Implementing a Flex Gateway Custom Policy in Rust](https://docs.mulesoft.com/gateway/policies-custom-flex-implement-rust)

You can find another example of a Rust policy for Anypoint Flex [here](https://github.com/jrhuerga/mule-flex-data-masking).

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

4. This will generate a file named _flex_custom_policy_auth_header.wasm_ in the folder _target/wasm32-unknown-unknown/release_ . You can optionally use wasm-gc to reduce its size. If necessary, you can use an AWS S3 bucket to copy that file so it will be available on your laptop to upload it to MuleSoft Exchange using a browser.




## Registering a Flex Gateway
There are three ways of registering a new Flex Gateway: using Linux binary, using Docker, and using Kubernetes. This policy will work seamless with any of the three choices. These steps describe how to register a new Flex Gateway using Docker.

1. Download and install the Flex Gateway container image:

    `$ docker pull mulesoft/flex-gateway:1.0.1`

2. Register Flex Gateway to Anypoint Platform by running the following command replacing <gateway-name> by your own value.

    `$ docker run --entrypoint flexctl -w /registration \`
    
    `-v "$(pwd)":/registration mulesoft/flex-gateway:1.0.1 \`
    
    `register <gateway-name> \`
    
    `--token=40e145a8-b49e-445a-af75-2303adcb9069 \`
    
    `--organization=a02dd3bb-28ff-4339-bafa-06f7f0332cc0 \`
    
    `--connected=true`

3. Run the following command to start the Flex Gateway replacing <absolute-path-to-directory-with-conf-file> with the path and the UUID in the name of the .conf file with the one created in the previous step.

    `$ docker run --rm \`

    `-v <absolute-path-to-directory-with-conf-file>/:/etc/flex-gateway/rtm \`

    `-p 8081:8081 \`

    `-e FLEX_RTM_ARM_AGENT_CONFIG=/etc/flex-gateway/rtm/<UUID-of-your-file>.conf \`

    `mulesoft/flex-gateway:1.0.1`




## Publishing the policy in Exchange
Following steps describe how to publish the policy in MuleSoft Exchange.

1. Open MuleSof Exchange and click on _Publish new asset_
1. Enter the name _Custom Auth Header_
1. Select _Policy_ as _Asset Type_
1. Upload the file _schema.json_
1. Upload the file _definition.yaml_. To avoid errors, make sure you keep its EOL characters in Unix format.
1. Click on _Publish_. This will take a while. When it finishes, click on Implementations and _Add Implementation_
1. Enter the name _Custom Auth Header Impl_
1. Upload the file _flex_custom_policy_auth_header.wasm_ you obtained in previous steps after compiling the sources using Rust and Cargo
1. Upload the file _implementation.yaml_. To avoid errors, make sure you keep its EOL characters in Unix format.
1. Click on _Add Implementation_
    

## Testing the policy
Once that the policy is published in Exchange, it will be ready to be applied to APIs. Follow these steps:

1. Publish an API Spec in Exchange
1. Define in API Manager a new API in the previously created API Gateway
1. Apply in that API instance the policy - defining any text as value - and wait some minutes until it is applied
1. Make a call to the API using postman. It should produce a 401 error
1. Repeat the call, but passing a new header named _x-custom-auth_ with the value you have defined in the policy. The API should produce an HTTP 200 status.


