

from Crypto.Cipher import AES
import hashlib
import sys
import binascii
import base64
from Crypto.Protocol.KDF import PBKDF2
from Crypto.Hash import SHA256
from Crypto.Random import get_random_bytes


plaintext='hello how are you?'
password='qwerty123'


if (len(sys.argv)>1):
  plaintext=(sys.argv[1])
if (len(sys.argv)>2):
  password=(sys.argv[2])

def encrypt(plaintext,key, mode):
  encobj = AES.new(key, AES.MODE_GCM)
  ciphertext,authTag=encobj.encrypt_and_digest(plaintext)
  return(ciphertext,authTag,encobj.nonce)

def decrypt(ciphertext,key, mode):
  print(ciphertext)
  (ciphertext,  authTag, nonce) = ciphertext
  encobj = AES.new(key,  mode, nonce)
  return(encobj.decrypt_and_verify(ciphertext, authTag))

#key = hashlib.sha256(password.encode()).digest()
#
#salt = get_random_bytes(32)
#key = PBKDF2(password, salt, 32, count=1000000, hmac_hash_module=SHA256)
#
#print("GCM Mode: Stream cipher and authenticated")
#print("\nMessage:\t",plaintext)
#print("Password:\t\t",password)
#
#
#ciphertext = encrypt(plaintext.encode(),key,AES.MODE_GCM)
#
#print("Salt:\t\t",binascii.hexlify(salt))
#print("Cipher:\t\t",binascii.hexlify(ciphertext[0]))
#print("Auth Msg:\t",binascii.hexlify(ciphertext[1]))
#print("Nonce:\t\t",binascii.hexlify(ciphertext[2]))

base64_str = "mu8BdsoEZLLv3DGuPLOHNnxAH4ouHeNG/lrOAINzIOG6hO8DSuTGU4V3uICaG00="
password = "AY5bRWdFUJpUmyPGtmdKuB85qVgMNQvD"

text = base64.b64decode(base64_str)

salt = text[0:16]
iv = text[17:28]
data = text[29:-1]

key =  PBKDF2(password, salt, 32, count=1000000, hmac_hash_module=SHA256)

res= decrypt(data,key,AES.MODE_GCM)


print ("\n\nDecrypted:\t",res.decode())
