# Password Splitter

Splits a password for the TCD for ease of reading, in order to reduce misreading and PW re-ehtries.

## Process

1. Read in password string, either from an EnvVar, CL argument, or hard-coded.
2. Parse through each character in PW
3. Place a **-** between characters, skipping any character **F** or **f**
4. Display final string for user to copy/read.
