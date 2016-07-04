var WebIDKey = 'https://kcorre.github.io/.well-known/webid/foaf.json'
var idp_addr = {'domain': "https://kcorre.github.io", 'protocol': 'WebID-proxy.js'}

function getWebID(){
  return new Promise((resolve, reject) => {
    var xmlhttp = new XMLHttpRequest()
    xmlhttp.onreadystatechange = () => {
      if (xmlhttp.readyState == 4 && xmlhttp.status == 200) {
        var res = JSON.parse(xmlhttp.responseText)
        res.error != undefined ? reject(res.error) : resolve(res)
      }
    }
    xmlhttp.open("GET", WebIDKey, true)
    xmlhttp.send()
  })
}

 function str2ab(str) {
   var buf = new ArrayBuffer(str.length);
   var bufView = new Uint8Array(buf);
   for (var i=0, strLen=str.length; i < strLen; i++) {
     bufView[i] = str.charCodeAt(i);
   }
   return buf;
 }

 function ab2str(buf) {
   return String.fromCharCode.apply(null, new Uint8Array(buf));
 }

 function getStoredKey(){
     return new Promise((resolve, reject) => {
         try{
             var request = indexedDB.open('WebIDKey')
             request.onsuccess = function(e){
                 idb = e.target.result
                 var transaction = idb.transaction('key_store', IDBTransaction.READ_ONLY);
                 var objectStore = transaction.objectStore('key_store');
                 var key;
                 objectStore.openCursor().onsuccess = function(event){
                     var cursor = event.target.result;
                     if (cursor){
                         //TODO Check for private key
                         key = cursor.value.value
                         cursor.continue();
                     } else {
                         resolve(key)
                     }
                 }
             }
         }catch(err){
            reject({'name': 'Key Store error'})
         }
     })
 }

 function signAssertion(contents, key, webId){
     return new Promise((resolve, reject) => {
         var header = {'typ': 'JWT', 'alg': 'RS256', 'jku': 'https://kcorre.github.io/.well-known/webid/foaf.json#key'}
         var payload = {'sub': webId.me,
                        'iss': webId.me,
                        'rtcsdp': contents}
         var jwt = btoa(JSON.stringify(header))+'.'+btoa(JSON.stringify(payload))

         crypto.subtle.sign({'name': 'RSASSA-PKCS1-v1_5'},
                            key.privateKey,
                            str2ab(jwt))
         .then(signature => {
             resolve({'assertion':jwt+'.'+btoa(ab2str(signature)), idp: idp_addr})
         })
     })
 }

// IDP Proxy code
var idp = {
  /**
  * Generation of an IdAssertion through OIDC IdP
  */
  generateAssertion: (contents /*, origin, hint */) => {
    var pKey = getStoredKey()
    var pWebID = getWebID()
    return Promise.all([pKey, pWebID])
    .then(results => signAssertion(contents, results[0], results[1]))
  },
  /**
  * Verification of a received IdAssertion validity
  * Can also be used to validate token received by IdP
  * @param  {DOMString} assertion assertion
  */
  validateAssertion: (assertion /*, origin */) => {
    assertion = assertion.split(".")
    var header = assertion[0],
        payload = assertion[1],
        signature = assertion[2]
    //TODO there is probably a better way to do that?
    signature = signature.replace(/_/g, "/").replace(/-/g, "+")
    return new Promise((resolve, reject) =>
      getWebID()
        .then(WebID =>
      crypto.subtle.importKey('jwk',WebID.key,{ name: 'RSASSA-PKCS1-v1_5',hash: {name: "SHA-256"}},true, ['verify'])
        .then(JWK =>
      //crypto.verify(algo, key, signature, text2verify);
      crypto.subtle.verify('RSASSA-PKCS1-v1_5',
                           JWK,
                           str2ab(atob(signature)),   //ArrayBuffer of the signature,
                           str2ab(header+"."+payload))//ArrayBuffer of the data
        .then(result => {
      if (!result) reject(new Error('Invalid signature on identity assertion'))
      else {
        var json = JSON.parse(atob(payload))
        resolve({'identity': WebID.nick+'@'+'kcorre.github.io', 'contents': json.rtcsdp})
      }})))
    )}
}

if (typeof rtcIdentityProvider != 'undefined') {
  rtcIdentityProvider.register(idp);
} else {
  console.warn('IdP not running in the right sandbox');
}
