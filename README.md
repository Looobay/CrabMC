# CrabMC

### Also guys if you see something wrong in the code about the Minecraft EULA please tell me.

## Why does this project exist?

* Because I like Rust.
* Because I like Minecraft.
* Because I heard Minecraft was slow and wanted to fix it in the server-side code (I don't know how).

## What are the goals of the project?

* To replace the vanilla Minecraft server.
* To be a blazing fast software!

## What are the features of the project now?

* 🟢: Create eula.txt and server.properties (EULA is mandatory to respect the Mojang license).
* 🟢: Manage logs (it was a pain).
* 🟢: Manage handshake package.
* 🟡: Manage connection packages (do not handle Mojang authentication).
* 🔴: Manage game packages.
* 🔴: Generate the world.
* 🔴: And the others things... (yes it's a big project)

## How to compile the project ?

### On Win32:
Install rustup and cargo on your Windows computer.

Clone the repo and enter `cargo build --release` and it's done!

### On Mac OS:
Install rustup and cargo on your macOS computer (Sadly I can't compile from windows to macOS so I use a macbook air).

Clone the repo and enter `cargo build --release` and it's done!

I recommend you to run the output file with `sudo ./CrabMC` it will work better than in normal mode.

### On Linux:
I didn't try for Linux so it should be the same steps as Win32 and macOS...

The Github action compiler succeed compiling the software on Ubuntu so it is probably ok...

Tell me if you try this!

## Can I contribute to the project?
Yes of course! You can contribute to the project if you think your code is useful, don't be shy.

But remember that this project is tiny, so its governance is BDFN (Benevolent Dictator For Now), which means that I (Looobay) have the last word on everything, but I'm a beginner in Rust so if you explain your code to me, I'll probably accept your contribution

If you do not agree with that system you can fork the project.

## Licenses

This project is under MIT license, you can read this in [LICENSE](LICENSE) file or at https://opensource.org/license/MIT.

You can see all the licenses of every dependency at [licenses.tsv](licenses.tsv).
