let input =
  "wJDT3o+k+kdjWh7Dq5xn8QjvKgE96AbKswMxr+d9dpVuiukmsxcILq3EbWAj4LGu2RNkog==";

let passphrase = "3EP1sqZP2G6bA8tmpVWlDSCyy4uPb1Cm";

const enc = new TextEncoder();

let encryptedDataBuff = Uint8Array.from(atob(input), (c) => c.charCodeAt(null));
const salt = encryptedDataBuff.slice(0, 16);
const iv = encryptedDataBuff.slice(16, 16 + 12);
const data = encryptedDataBuff.slice(16 + 12);

function getPasswordKey(password) {
  return window.crypto.subtle.importKey(
    "raw",
    enc.encode(password),
    "PBKDF2",
    true,
    ["deriveKey"]
  );
}

function deriveKey(passwordKey, salt, keyUsage) {
  return window.crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt,
      iterations: 250000,
      hash: "SHA-256",
    },
    passwordKey,
    { name: "AES-GCM", length: 256 },
    true,
    keyUsage
  );
}

const passwordKey = await getPasswordKey(passphrase);
const aesKey = await deriveKey(passwordKey, salt, ["decrypt"]);

const decryptedContent = await window.crypto.subtle.decrypt(
  {
    name: "AES-GCM",
    iv: iv,
  },
  aesKey,
  data
);

let passwordKeyExported = await crypto.subtle.exportKey("raw", passwordKey);
let aesKeyExported = await crypto.subtle.exportKey("raw", aesKey);
