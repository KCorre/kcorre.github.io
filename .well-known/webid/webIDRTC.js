var indexedDB = window.indexedDB || window.webkitIndexedDB || window.msIndexedDB;
var IDBKeyRange = window.IDBKeyRange || window.webkitIDBKeyRange;
var openCopy = indexedDB && indexedDB.open;

var IDBTransaction = window.IDBTransaction || window.webkitIDBTransaction;

if (IDBTransaction)
{
    IDBTransaction.READ_WRITE = IDBTransaction.READ_WRITE || 'readwrite';
    IDBTransaction.READ_ONLY = IDBTransaction.READ_ONLY || 'readonly';
}
/***
 * Create database snippet
 * */
function createDB(){

console.log('Hello!')
  var request = indexedDB.open('WebIDKey');
  request.onupgradeneeded = function(e)
  {
      // e is an instance of IDBVersionChangeEvent
      var idb = e.target.result;
      console.log(idb)
      if (!idb.objectStoreNames.contains('key_store')){
      //    idb.deleteObjectStore('key_store');
        var store = idb.createObjectStore('key_store', {keyPath: 'key', autoIncrement: true});
      // createIndex operations possible to be pefromed on store.createIndex
        store.createIndex('by_key', 'key_store', {unique: true, multiEntry: false});
      }
  };

  request.onsuccess = function(e) {    console.log('Created DB') };
  request.onerror = function(e) { console.lgo('Failed to create DB') };
}

var dropDatabase = function()
{
    console.log('Bye!')
    var request = indexedDB.deleteDatabase('WebIDKey');
    request.onsuccess = function() {console.log('Drop Succeeded') };
    request.onerror = function() { console.log('Drop Failed') };
    request.onblocked = function () {
        console.log("Couldn't delete database due to the operation being blocked");
    };
};

function registerKey(key){
  var request = indexedDB.open('WebIDKey');
  request.onsuccess = function(e)
  {
    var idb = e.target.result;
    var trans = idb.transaction('key_store', IDBTransaction.READ_WRITE);
    var store = trans.objectStore('key_store');
    // add
    console.log(key)
    var requestAdd = store.add({'key': 'jwk', 'value': key});
    requestAdd.onsuccess = function(e) {
      console.log('success storing key')
    };

    requestAdd.onfailure = function(e) {
      console.log('failure storing key')
    };
  };
}

function getStoredKey(){
  var request = indexedDB.open('WebIDKey');
  request.onsuccess = function(e)
  {
      idb = e.target.result;
      var transaction = idb.transaction('key_store', IDBTransaction.READ_ONLY);
      var objectStore = transaction.objectStore('key_store');

      objectStore.openCursor().onsuccess = function(event)
      {
          var cursor = event.target.result;
          if (cursor)
          {
              document.getElementById('stored').innerText = cursor.value.value
              console.log(cursor.value.value)
              cursor.continue();
          }
          else
          {
              console.log('Entries all displayed.');
          }
      };
  };
}

function generatePubKey(){
    window.crypto.subtle.generateKey(
        {
            name: "RSASSA-PKCS1-v1_5",
            modulusLength: 2048, //can be 1024, 2048, or 4096
            publicExponent: new Uint8Array([0x01, 0x00, 0x01]),
            hash: {name: "SHA-256"}, //can be "SHA-1", "SHA-256", "SHA-384", or "SHA-512"
        },
        true, //whether the key is extractable (i.e. can be used in exportKey)
        ["sign", "verify"] //can be any combination of "sign" and "verify"
    )
    .then(function(key){
         window.crypto.subtle.exportKey("jwk", key.privateKey)
         .then(jwk => document.getElementById('generatedPri').innerText = JSON.stringify(jwk))
         window.crypto.subtle.exportKey("jwk", key.publicKey)
         .then(jwk => document.getElementById('generatedPub').innerText = JSON.stringify(jwk))

         registerKey(key)
    })
    .catch(function(err){
        console.error(err);
    });
}
