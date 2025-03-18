from urllib.request import urlopen

def lambda_handler(event, context):
    print("[handler] Sending request to https://aws.amazon.com")

    try:
        response = urlopen("https://aws.amazon.com")
        print("[handler] Got response code", response.getcode())
    except Exception as e:
        print("[handler] Got error", e)

    return 'done'

