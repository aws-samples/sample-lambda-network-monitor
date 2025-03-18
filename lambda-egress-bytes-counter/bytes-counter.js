import fs from 'fs';

let prevInvokeData = {};

// Subject to change, make sure you scan all devices
const DEVICE_NAME = 'vint_runtime';
function countEgressBytes() {
    const newData = getNetworkDeviceData(DEVICE_NAME);
    const lastRxBytes = prevInvokeData.rxBytes ?? 0;
    const lastTxBytes = prevInvokeData.txBytes ?? 0;

    const rxBytesDiff = newData.rxBytes - lastRxBytes;
    const txBytesDiff = newData.txBytes - lastTxBytes;
    console.log(`[Network data] thisInvokeRxBytes=${rxBytesDiff} thisInvokeTxBytes=${txBytesDiff}`);
    console.log(`[Network data] totalRxBytes=${newData.rxBytes} totalTxBytes=${newData.txBytes}`);
    printCloudWatchEMF(rxBytesDiff, txBytesDiff);
    prevInvokeData = newData;
}

function getNetworkDeviceData(device) {
    const file = fs.readFileSync('/proc/net/dev', 'utf8');
    const lines = file.split('\n');
    const line = lines.filter((line) => line.includes(device))[0];
    const parts = line.trim().split(/\s+/g);

    const name = parts[0].substring(0, parts[0].length - 1);
    const rxBytes = parseInt(parts[1]);
    // const rxPackets = parseInt(parts[2]);
    // const rxErrors = parseInt(parts[3]);
    // const rxDrop = parseInt(parts[4]);
    // const rxFifo = parseInt(parts[5]);
    // const rxFrame = parseInt(parts[6]);
    // const rxCompressed = parseInt(parts[7]);
    // const rxMulticast = parseInt(parts[8]);
    const txBytes = parseInt(parts[9]);
    // const txPackets = parseInt(parts[10]);
    // const txErrors = parseInt(parts[11]);
    // const txDrop = parseInt(parts[12]);
    // const txFifo = parseInt(parts[13]);
    // const txColls = parseInt(parts[14]);
    // const txCarrier = parseInt(parts[15]);
    // const txCompressed = parseInt(parts[16]);
    return {
        name,
        rxBytes,
        txBytes,
    };
}

function printCloudWatchEMF(rxBytes, txBytes) {
    const json = {
        "_aws": {
            Timestamp: Date.now(),
            CloudWatchMetrics: [{
                Namespace: "LambdaNetworkMonitor",
                Dimensions: [["functionName"]],
                Metrics: [{
                    Name: "rxBytes",
                    Unit: "Bytes"
                },{
                    Name: "txBytes",
                    Unit: "Bytes"
                }]
            }]
        },
        functionName: process.env.AWS_LAMBDA_FUNCTION_NAME,
        rxBytes,
        txBytes,
    }

    console.log(JSON.stringify(json));
}

const handlerWrapper = (handler) => {
    return async (event, ctx) => {
        const handlerResult = await handler(event, ctx);
        countEgressBytes();
        return handlerResult;
    }
};

export {handlerWrapper, countEgressBytes};