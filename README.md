# LastPass Blob Deobfuscator

A tool to deobfuscate your LastPass blob and determine if any passwords are encrypted with AES-ECB and therefore more vulnerable with the recent breach.

## Usage

```
cargo install lpass-blob
# Grab your blob: https://www.grc.com/sn/SN-904-Notes.pdf
# Copy & paste it to a text file
lpass-blob [path to the text file]
```

---

Created because of Steve Gibson's call-to-action on the [Security Now! Podcast](https://grc.com/sn). I don't use LastPass, but made an account for testing.

I'd like to take this opportunity to plug [KeePassXC](https://keepassxc.org/) (my password manager of choice) and [pass](https://www.passwordstore.org/) (the standard UNIX password manager).

If you find any bugs, please report via Github issues.

