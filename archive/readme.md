**rspotify looks for these env vars if no external env supplied (i.e. file)**

Set RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET in an .env file (after
enabling the `env-file` feature) or export them manually:

export RSPOTIFY_CLIENT_ID="your client_id"
export RSPOTIFY_CLIENT_SECRET="secret"

These will then be read with `from_env`.

Otherwise, set client_id and client_secret explictly:

env::set_var("RSPOTIFY_CLIENT_ID", "291fcee697f94999bf71fbf499bdfb54");
env::set_var("RSPOTIFY_CLIENT_SECRET", "439e720432e149c0bb3176a3b5e1713c");
env::set_var("RSPOTIFY_REDIRECT_URI", "https://example.org/callback");
