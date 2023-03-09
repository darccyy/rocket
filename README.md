# Kahoot Renaming Server

Kahoot renaming API. Use [Redirector] addon to use custom name for Kahoot generated name.

Hosted on [Railway] with Rust (Just a little program). In the future it will most likely be moved to a new domain.

# Usage

## Kahoot

- Install Redirector addon on Firefox or Chrome
- Click the Redirector button (Addon) -> 'Edit Redirects'
- 'Create new redirect'
- 'Example URL': https://apis.kahoot.it/namerator
- 'Include Pattern': https://apis.kahoot.it/namerator
- 'Redirect to': https://kahoot-name.up.rocket.app/<NAME> with <NAME> being the name you want (Refer to [API Usage])
- Click 'Show advanced options'
- Make sure 'XMLHttpRequests (Ajax)' is checked
- Click Save

It should automatically change the name once you spin

## API Usage

Fetch name: https://kahoot-name.up.rocket.app/<NAME>. If no name is given, default will be used. Add the caret (^) character to represent an invisible character. This will not show on the screen, and will bypass the profanity filter
