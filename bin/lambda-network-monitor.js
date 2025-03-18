#!/usr/bin/env node

const cdk = require('aws-cdk-lib');
const { ProcfsStack } = require('../stacks/procfs-stack');
const { LdPreloadStack } = require('../stacks/ld-preload-stack');

const app = new cdk.App();
new ProcfsStack(app, 'LambdaNetworkMonitor-Procfs');
new LdPreloadStack(app, 'LambdaNetworkMonitor-LdPreload');
