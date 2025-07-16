# STM32 Protobuf USB

This project demonstrates how a STM32 can be programmed to receive and process
custom USB commands using a binary protocol like Protobuf.

It aims to solve an issue we had at work to programmatically control a custom
piece of dumb hardware. We didn't end up using an MCU in the final product,
but I thought it was an interesting idea, so the project serves as a simple
demonstration.

Check out the companion desktop application which demonstrates how to communicate
with a STM32 who has had this program flashed:

[STM32 USB Desktop Comms](https://github.com/atan2l/Stm32UsbDesktopComms)
