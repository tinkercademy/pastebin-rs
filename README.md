# A simple Rust-based pastebin

Very naive. No rate limiting, no authentication.

Also, you can definitely upload illegal content, because it's
not checked.

Possible improvements:
- [ ] MIME type detection
- [ ] Rate limiting
- [ ] Automatically deleting old pastes

License: AGPL 

## API Endpoints

### GET v1/paste/:id
Returns the contents of a paste.

### POST v1/paste

Creates a new paste.

body: Your paste contents

E.g.

```shell
$ cat README.md | curl -X POST -d @- http://localhost:3000/v1/paste
```
