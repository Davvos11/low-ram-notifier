# Low RAM notifier
Sends a notification if you are low on available RAM

## Usage:
`low-ram-notifier [bytes]` will warn you below [bytes] bytes of available RAM.

You can set the refresh interval with `--interval [interval]`, where [interval] is the time in ms.

Notifications look like this (on my system):
[screenshot]

I only tested this on Linux, but the libraries used claim to also support Windows and MacOS.