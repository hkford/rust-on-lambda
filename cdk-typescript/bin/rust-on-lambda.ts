#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { RustOnLambdaStack } from '../lib/rust-on-lambda-stack';

const app = new cdk.App();
new RustOnLambdaStack(app, 'RustOnLambdaStack', {
    env: { region: 'us-east-1' },
});