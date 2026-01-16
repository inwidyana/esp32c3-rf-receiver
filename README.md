# ESP32-C3 RF RECEIVER

This simple project could receive RF 433 Mhz signal and display it as a graph

## Hardware Setup

To run this project, we need:
- ESP32 C3 Ultra Mini
- RF 433 Mhz Receiver (tested with FS1000A green board)

Wiring setup:
| ESP32 C3 | Receiver |
| -------- | -------- |
| 5V       | VCC      |
| GND      | GND      |
| GPIO 4   | One pin* |
*Only one pin from FS1000A receiver is needed. The other could be ignored

## Running
To run on windows run (in powershell):
```bash
usbipd attach --wsl --busid 1-10 --auto-attach
```
with `1-10` being the busid from:
```bash
usbipd list
```

Then run on this repo:
```bash
cargo run
```
RF 433 Mhz signals should be printed out as graphs.
