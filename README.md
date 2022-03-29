# Rust on AWS Lambda Example

This repository contains example code of AWS Lambda function written in Rust. You can try deploying Lambda function with AWS CDK or AWS SAM CLI.

```text
- cdk-typescript # CDK template
- lambda # Lambda function code written in Rust
- sam # SAM template
```

## Deploy with AWS CDK

### Prerequisites

- AWS Account
- AWS CLI
- Docker daemon
- Node.js

### Deploy commands

```shell
# Run the following commands
$ cd cdk-typescript
$ npm install
$ cdk deploy
```

Copy the name of Lambda function.

### Invoke Lambda function in the cloud

```shell
# Invoke Lambda function in the cloud
$ aws lambda invoke --cli-binary-format raw-in-base64-out \ 
--function-name <Lambda function name>  \
--payload '{ "message": "AWS Lambda on Rust" }' \
output.json
```

### Delete

```shell
# Delete CDK stack
$ cdk destroy
```

## Deploy with AWS SAM CLI

### Prerequisites

- AWS Account
- AWS CLI
- Docker daemon
- SAM CLI

### Build command

```shell
# Run the following command to build Docker image for AWS Lambda.
$ cd sam
$ sam build
```

### Invoking Lambda function locally

```shell
# With correct event
$ sam local invoke RustFunction -e events/with_message.json
{"message":"Hello, AWS Lambda on Rust!"}

# With incorrect event
$ sam local invoke RustFunction -e events/without_message.json
{"errorType":"&alloc::boxed::Box<dyn std::error::Error+core::marker::Send+core::marker::Sync>","errorMessage":"No message provided"}
```

### Deploy Lambda function

```shell
$ sam deploy --guided

Stack Name [rust-on-lambda]: rust-on-lambda
AWS Region [us-east-1]: us-east-1
Confirm changes before deploy [Y/n]: y
Allow SAM CLI IAM role creation [Y/n]: y
Disable rollback [y/N]: n
Save arguments to configuration file [Y/n]: y
SAM configuration file [samconfig.toml]: samconfig.toml
SAM configuration environment [default]: default
Deploy this changeset? [y/N]: y
```

Copy the name of Lambda function.

### Invoke Lambda function in the cloud

```shell
# Invoke Lambda function in the cloud
$ aws lambda invoke --cli-binary-format raw-in-base64-out \ 
--function-name <Lambda function name>  \
--payload '{ "message": "AWS Lambda on Rust" }' \
output.json
```

### Delete

```shell
# Delete CloudFormation stack
$ sam delete

Are you sure you want to delete the stack rust-on-lambda in the region us-east-1 ? [y/N]: y
Are you sure you want to delete the folder rust-on-lambda in S3 which contains the artifacts? [y/N]: y
Do you you want to delete the ECR companion stack <stack name> in the region us-east-1 ? [y/N]: y
ECR repository <repository name> may not be empty. Do you want to delete the repository and all the images in it ? [y/N]: y
```
