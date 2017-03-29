### WebRTC IdP Proxy for WebID like authentication

The associated files demonstrate how to use and IdP Proxy from WebRTC (https://www.w3.org/TR/webrtc/#sec.identity-proxy)
based on a WebID-like authentication mechanism.

1. Generate and Store private key
Access .well-known/webid/ to generate and store a CryptoKey in IndexedDB for origin kcorre.github.io

2. Upload key
Upload the public key to the WebID resource. Here we use a JSON-WebID-like object at .well-known/webid/foaf.json

3. Generate Assertion
The IdP Proxy (from origin kcorre.github.io) use the CryptoKey in IndexedDB to sign a JWT.

3. Validate Assertion
The IdP Proxy access foaf.json to get the public key and validate the receive JWT. 
