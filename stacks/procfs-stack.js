const {Stack, Duration} = require('aws-cdk-lib');
const lambda = require('aws-cdk-lib/aws-lambda');
const {Architecture, Runtime} = require("aws-cdk-lib/aws-lambda");

class ProcfsStack extends Stack {
    constructor(scope, id, props) {
        super(scope, id, props);

        // This function demonstrates reading network stats from /proc/net/dev
        new lambda.Function(this, 'ProcNetDev', {
            functionName: 'proc-net-dev-demo-function',
            runtime: Runtime.NODEJS_22_X,
            architecture: Architecture.X86_64,
            handler: 'index.handler',
            memorySize: 256,
            timeout: Duration.seconds(10),
            code: lambda.Code.fromAsset('./lambda-egress-bytes-counter')
        });

    }
}


module.exports = {ProcfsStack}
