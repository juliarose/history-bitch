# history-bitch

Chrome extension for viewing trade history.

## Legal

Offered under the [GNU General Public License v2.0](LICENSE). It is offered as-is and without liability or warranty. You are free to modify and redistribute this extension as long as the source is disclosed and it is published under the same license.

History Bitch is not affiliated with Steam or Valve.

## Privacy Policy

This extension requires permissions to <https://steamcommunity.com/dev/apikey> and <https://api.steampowered.com/IEconService/GetTradeHistory> for loading trade history for your Steam account, as well as data storage to your disk. Stored data is entirely local and not shared anywhere outside of the extension.

**If API keys are stored in local storage.

In order to load trade history, the extension must use your API key. For reasons related to how data is stored in extensions, this is not 100% secure in keeping your API key private. However, access to your API key does not pose harm to your account other than a third party performing requests not on your behalf, the most harmful of which is [acting on trade offers](https://marketplace.tf/blog/posts/YHLZOB), which cannot take items from your account without mobile confirmation as well. Extensions run within their own sandbox. Their data cannot be directly accessed by outside sources with the exception of extensions with access to other extensions, providing protection against [XSS](https://owasp.org/www-community/attacks/xss/). Malware running on your computer or anyone with access to your computer could gain access. With that in mind, use at your own risk.