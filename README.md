> [!IMPORTANT]
> This project is no longer maintained.  
> Please check out the new project provided by the same author:  
> https://github.com/takuma-shishido/helix-rich-lsp  

helix-discord-presence is an lsp project that add support [Discord Rich Presence](https://discord.com/developers/docs/rich-presence/how-to) in [Helix](https://github.com/helix-editor/helix).

Since Helix does not currently support extensions ([a draft PR exists](https://github.com/helix-editor/helix/pull/8675)), I am using an alternative method with lsp.  
Once extensions are supported, I plan to switch from lsp to another method.

## Preview
<img width="270" alt="rich-presence-example" src="https://github.com/user-attachments/assets/4cdf2b12-5c18-4b97-bdb5-a06ccc766f0b" />

## How to install?

1. Copy `languages.toml` to `~/.config/helix/languages.toml`.

    ```sh
    cp languages.toml ~/.config/helix/languages.toml
    ```

2. Build this Rust project and add the binary to your PATH.

    ```sh
    cargo build --release
    export PATH=$PATH:/path/to/helix-discord-presence/target/release
    ```

    It is recommended to add the above `export` command to your `~/.bashrc` or `~/.zshrc` to automatically include the binary in your PATH when the shell starts.

    ```sh
    echo 'export PATH=$PATH:/path/to/helix-discord-presence/target/release' >> ~/.bashrc
    # or
    echo 'export PATH=$PATH:/path/to/helix-discord-presence/target/release' >> ~/.zshrc
    ```

3. Start Helix and verify that the Discord presence is displayed correctly.

## Currently recognized issues (Please do not create issues about this)
- The "View Repository" button is not displayed (This seems to be a Discord issue. There is nothing I can do about it).

## TODO
- [ ] Support configuration (currently hardcoded)
- [ ] Add idle mode

## Credit
[zed-discord-presence](https://github.com/xhyrom/zed-discord-presence): A lot of code was written with reference to this project. Thank you
