# CrabMC

### Also guys if you see something wrong in the code about the Minecraft EULA please tell me.

### The code of [packet_handler.rs](src/network/packet_handler.rs) is a mess we need to rewrite it ; it's too painfull to add features...

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

## How to install the software ?

### You can install the software by checking in the [releases](https://github.com/Looobay/CrabMC/releases) page, here you can download a nightly release (latest build) or a release ("safest" build).

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

Just please one tip if you want to contribute: make sure you understand how the software work by reading ["how it work ?"](doc/how_it_work.md) in the doc directory so if you don't understand just try to send me a message!

But remember that this project is tiny, so its governance is BDFN (Benevolent Dictator For Now), which means that I (Looobay) have the last word on everything, but I'm a beginner in Rust so if you explain your code to me, I'll probably accept your contribution

If you do not agree with that system you can fork the project.

## Licenses

This project is under the Mozilla Public License 2.0, you can read this in [LICENSE](LICENSE) file or at https://www.mozilla.org/en-US/MPL/2.0/.

You can see all the licenses of every dependency at [licenses.tsv](licenses.tsv).
