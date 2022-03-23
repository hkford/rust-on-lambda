import { Stack, StackProps, CfnOutput } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { aws_lambda as lambda, aws_logs as logs } from 'aws-cdk-lib';
import * as path from 'path';

export class RustOnLambdaStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);
        const lambdaFunction = new lambda.DockerImageFunction(this, 'Function', {
            code: lambda.DockerImageCode.fromImageAsset(path.join(__dirname, '../../lambda')),
            architecture: lambda.Architecture.X86_64,
            logRetention: logs.RetentionDays.ONE_DAY
        });

        new CfnOutput(this, 'FunctionName', {
            value: lambdaFunction.functionName
        });
    }
}