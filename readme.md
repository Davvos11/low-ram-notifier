# Low RAM notifier
Sends a notification if you are low on available RAM

## Download:
Download the latest version [here](https://github.com/Davvos11/low-ram-notifier/releases/).

## Usage:
`low-ram-notifier [bytes]` will warn you below [bytes] bytes of available RAM.

You can set the refresh interval with `--interval [interval]`, where [interval] is the time in ms.

Notifications look like this (on my system):

![Screenshot_20240105_183031](https://github.com/Davvos11/low-ram-notifier/assets/20478740/df692162-5ea1-4bc3-9a72-0136c0627733)

## Auto-start:
It might be easiest to use your desktop manager to start the program.

As an alternative there is also a systemd service provided. You can install it like so:  
1. First, edit `low-ram-notifier@.service`, you can set your preferences in the `ExecStart` line (default is 1GB).
2. ```shell
   sudo cp ./low-ram-notifier /usr/local/bin
   sudo cp ./low-ram-notifier@.service /etc/systemd/system
   sudo systemctl daemon-reload
   sudo systemctl enable low-ram-notifier@$(id -u)
   sudo systemctl start low-ram-notifier@$(id -u)
   ```

You should immediately get a test notification, if the service properly started.
