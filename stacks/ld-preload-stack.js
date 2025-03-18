const {Stack, Duration, RemovalPolicy} = require('aws-cdk-lib');
const lambda = require('aws-cdk-lib/aws-lambda');
const s3 = require('aws-cdk-lib/aws-s3');
const {Architecture, Runtime} = require("aws-cdk-lib/aws-lambda");
const fs = require('fs');

class LdPreloadStack extends Stack {
    constructor(scope, id, props) {
        super(scope, id, props);

        // Make sure you run `cargo build` first to build the .so
        // Copying .so to the Lambda layer folder
        // The LD_PRELOAD environment variable is set in the 'wrapper' file
        try {
            fs.copyFileSync(
                './lambda-network-monitor-rust/target/debug/liblambda_network_monitor.so',
                './lambda-network-monitor-layer/liblambda_network_monitor.so');
            console.log('Copied liblambda_network_monitor.so');
        } catch (e){
            console.error("Failed to copy liblambda_network_monitor.so (ignore if not deploying the LD_PRELOAD stack)");
        }

        // Creating a lauer
        const layer = new lambda.LayerVersion(this, 'MyLayer', {
            compatibleArchitectures: [Architecture.X86_64],
            layerVersionName: 'lambda-network-monitor-layer',
            code: lambda.Code.fromAsset('./lambda-network-monitor-layer')
        });

        // This function demonstrates using the Lambda network monitor layer in a nodejs function
        const nodeDemoFn = new lambda.Function(this, 'NodeDemoFunction', {
            functionName: 'lambda-network-monitor-demo-function-nodejs',
            runtime: Runtime.NODEJS_22_X,
            architecture: Architecture.X86_64,
            handler: 'index.handler',
            memorySize: 256,
            code: lambda.Code.fromAsset('./lambda-network-monitor-demo-functions/nodejs'),
            layers: [layer],
            timeout: Duration.seconds(10),
            environment: {
                "AWS_LAMBDA_EXEC_WRAPPER": "/opt/wrapper"
            }
        });

        // This function demonstrates using the Lambda network monitor layer in a python function
        const pythonDemoFn = new lambda.Function(this, 'PythonDemoFunction', {
            functionName: 'lambda-network-monitor-demo-function-python',
            runtime: Runtime.PYTHON_3_13,
            architecture: Architecture.X86_64,
            handler: 'main.lambda_handler',
            memorySize: 256,
            code: lambda.Code.fromAsset('./lambda-network-monitor-demo-functions/python'),
            layers: [layer],
            timeout: Duration.seconds(10),
            environment: {
                "AWS_LAMBDA_EXEC_WRAPPER": "/opt/wrapper"
            }
        });
    }
}

module.exports = {LdPreloadStack}
