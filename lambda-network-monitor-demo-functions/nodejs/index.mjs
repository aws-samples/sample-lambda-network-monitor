export const handler = async (event, context) => {
    try {
        console.log('[handler] Sending request to https://aws.amazon.com');
        const resp = await fetch("https://aws.amazon.com");
        console.log(`[handler] Got response code ${resp.status}`);
    } catch (e) {
        console.error(`[handler] Got error ${e}`);
    }

    return 'done';
}