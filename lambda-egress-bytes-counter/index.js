import {countEgressBytes, handlerWrapper} from './bytes-counter.js';

export const handler = async (event) => {

    // Simple fetch
    console.log('fetching...');
    await fetch('https://aws.amazon.com');
    console.log('fetch success');

    // Print bytes
    countEgressBytes();
};

// Alternatively, can also be used as a handler wrapper without explicit calling to countEgressBytes();
// export const handler = handlerWrapper(async(event, ctx)=>{ .... });
