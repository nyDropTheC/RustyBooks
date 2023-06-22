# a rusty backend for BookApp

Provides a backend for [BookApp](https://github.com/nyDropTheC/BookApp), giving it a secure way to request completions from OpenAI.

## Running it

* Deploy [FirebaseVerifier](https://github.com/nyDropTheC/FirebaseVerifier) on Cloudflare Workers
* Correct the service binding for FirebaseVerifier in wrangler.toml, if incorrect
* Add the necessary secrets (OPENAI_KEY and VERIFIER_URI, both are self-explanatory)
* ```npx wrangler deploy```

## Issues

* The code is trash