# About

This repository contains sample code for you to get started with the Channels application in IOTA Streams.

You can find documentation for these examples on the [IOTA documentation portal](https://docs.iota.org/docs/channels/introduction/get-started.md).

# Goal
The goal of this example is to show how a High-level API can reduce development complexity when working with IOTA-channels.
The two classes `channel_lite_writer` and `channel_lite_reader` fix the Tangle as transport medium for the stream, removing the need for the developer to set variables such as Minimum Weight Maginitude. 

# How It Works

Use `channel.open()` to open the channel and get the announcement verifier <br />
Use `channel.get_adderss()` to get the address of the channel (needed yb the reader to connect)<br />
Use `channel.write()` to write a message(public or masked) into the channel <br />
<br />
Use `channel.connect()` to connect to a channel<br />
Use `channel.read()` to read a message from the channel<br />

# Try it yourself
Clone the repo: <br />
`git clone https://github.com/AleBuser/channels-examples`<br />
Open the Folder:<br />
`cd channels-examples`<br />
Run the code:<br />
`cargo run`<br />

# Todo
- Test subscription once bundle inconsistency bug is solved <br />
- Implement remove Subscriber <br />
