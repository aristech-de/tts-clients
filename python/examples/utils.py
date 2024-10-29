import os
from dotenv import load_dotenv

load_dotenv()
host = os.getenv("HOST")
auth_token = os.getenv("TOKEN")
auth_secret = os.getenv("SECRET")
# If specified, load the contents of the file as the root certificate
root_cert = os.getenv("ROOT_CERT", "")
if root_cert:
  with open(root_cert, "rb") as f:
    root_cert = f.read()
# If SSL is not explicitly set to True, we set ssl to true if auth_token or auth_secret are set or if a root certificate is provided.
ssl = os.getenv("SSL", "false").lower() == "true" or len(auth_token) != 0 or len(auth_secret) != 0 or len(root_cert) != 0

voice_id = os.getenv("VOICE_ID", "anne_en_GB")

__all__ = ["host", "auth_token", "auth_secret", "root_cert", "ssl", "voice_id"]