# Kahoot Renaming Server

Kahoot renaming API. Use [Redirector](https://addons.mozilla.org/en-GB/firefox/addon/redirector/) addon to use custom name for Kahoot generated name.

Hosted on [Railway](https://railway.app) with Rust (Just a little program). In the future it will most likely be moved to a new domain.

# Usage

## Kahoot

- Install [Redirector](https://addons.mozilla.org/en-GB/firefox/addon/redirector/) addon on Firefox or Chrome
- Click the Redirector button (Addon) -> 'Edit Redirects'
- 'Create new redirect'
- 'Example URL': `https://apis.kahoot.it/namerator`
- 'Include Pattern': `https://apis.kahoot.it/namerator`
- 'Redirect to': `https://kahootname.up.rocket.app/<NAME>` with `<NAME>` being the name you want (Refer to [API Usage](#api-usage))
- Click 'Show advanced options'
- Make sure 'XMLHttpRequests (Ajax)' is checked
- Click Save

It should automatically change the name once you spin

## API Usage

Url syntax options:

- `https://kahootname.up.rocket.app/` (Default name)
- `https://kahootname.up.rocket.app/<NAME>` Path
- `https://kahootname.up.rocket.app/?name=<NAME>` Query overrides path

If no name is given, default will be used.

Add the caret (`^`) character to represent an invisible character.
This will not show on the screen, and will bypass the profanity filter.
If name only contains carets, then name will display blank.

# Issues

[Create a new issue](https://github.com/darccyy/kahoot-name/issues/new)
