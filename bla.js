const crypto = require("crypto").webcrypto;

const enc = new TextEncoder();
function atob(a) {
  return new Buffer(a, "base64").toString("binary");
}
const getPasswordKey = (password) =>
  crypto.subtle.importKey("raw", enc.encode(password), "PBKDF2", false, [
    "deriveKey",
  ]);

const deriveKey = (passwordKey, salt, keyUsage) =>
  crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt,
      iterations: 250000,
      hash: "SHA-256",
    },
    passwordKey,
    { name: "AES-GCM", length: 256 },
    false,
    keyUsage
  );

const bufferToBase64 = (buff) => btoa(String.fromCharCode.apply(null, buff));
const base64ToBuffer = (b64) =>
  Uint8Array.from(atob(b64), (c) => c.charCodeAt(null));
//
//export function generateRandomChars(number: number): string {
//  return Array(number)
//    .fill("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
//    .map(
//      (char) =>
//        char[
//          Math.floor(
//            (crypto.getRandomValues(new Uint32Array(1))[0] / (0xffffffff + 1)) *
//              char.length
//          )
//        ]
//    )
//    .join("");
//}
//
//export function generatePassphrase(): string {
//  return generateRandomChars(32);
//}
//
//export async function encryptData(
//  plainText: string,
//  passphrase: string
//): Promise<string> {
//  try {
//    const salt = crypto.getRandomValues(new Uint8Array(16));
//    const iv = crypto.getRandomValues(new Uint8Array(12));
//    const passwordKey = await getPasswordKey(passphrase);
//    const aesKey = await deriveKey(passwordKey, salt, ["encrypt"]);
//    const encryptedContent = await crypto.subtle.encrypt(
//      {
//        name: "AES-GCM",
//        iv: iv,
//      },
//      aesKey,
//      new TextEncoder().encode(plainText)
//    );
//
//    const encryptedContentArr = new Uint8Array(encryptedContent);
//    const buff = new Uint8Array(
//      salt.byteLength + iv.byteLength + encryptedContentArr.byteLength
//    );
//    buff.set(salt, 0);
//    buff.set(iv, salt.byteLength);
//    buff.set(encryptedContentArr, salt.byteLength + iv.byteLength);
//    const base64Buff = bufferToBase64(buff);
//
//    return base64Buff;
//  } catch (e) {
//    console.log(`Error - ${e}`);
//    return "";
//  }
//}

async function decryptData(encryptedText, passphrase) {
  try {
    const encryptedDataBuff = base64ToBuffer(encryptedText);
    console.log(encryptedDataBuff);
    const salt = encryptedDataBuff.slice(0, 16);
    console.log(salt);
    const iv = encryptedDataBuff.slice(16, 16 + 12);
    console.log(iv);
    const data = encryptedDataBuff.slice(16 + 12);
    console.log("asdfasdfasd " + data);
    const passwordKey = await getPasswordKey(passphrase);
    console.log(passwordKey);
    const aesKey = await deriveKey(passwordKey, salt, ["decrypt"]);
    console.log(aesKey);
    const decryptedContent = await crypto.subtle.decrypt(
      {
        name: "AES-GCM",
        iv: iv,
      },
      aesKey,
      data
    );

    return new TextDecoder().decode(decryptedContent);
  } catch (e) {
    console.log(`Error - ${e}`);
    return "";
  }
}

(async () => {
  let base64_str =
    "mu8BdsoEZLLv3DGuPLOHNnxAH4ouHeNG/lrOAINzIOG6hO8DSuTGU4V3uICaG00=";
  let password = "AY5bRWdFUJpUmyPGtmdKuB85qVgMNQvD";
  let out = await decryptData(base64_str, password);
  console.log(out);
})();
