# Practical Tech Challenge

I have created a simple hello work application based on the stm32f4 discovery board which has a board implementation in Renode.
I chose to use Renode over QEMU to knock out two challenge options at the same time.

This example was created and run on a Windows platform using Windows Subsystem for Linux to build.

This example uses the stm32 embedded_hal crate and examples to configure usart1 and send "Hello Dojo"

Assuming you have a up to date rust development environment, building the example is as simple as
`cargo build --release`

The resulting .elf file is located in `hello-dojo/target/thumbv7em-none-eabihf/release/hello-dojo`

If a rust environment is not configured, you can follow the instructions to get set up [here](https://docs.rust-embedded.org/book/intro/install.html). This example uses the thumbv7em-none-eabihf target

Renode can be installed for your system from [here](https://renode.io/#downloads).

To run the example, start your renode instance. Once the monitor screen is ready the following commands can be entered to configure the emulation and run the example
```
(monitor) mach create
(machine-0) machine LoadPlatformDescription @platforms/boards/stm32f4_discovery-kit.repl
(machine-0) showAnalyzer sysbus.usart1
(machine-0) sysbus LoadELF @<path to elf>
(machine-0) start
```
The result will be "Hello Dojo" in the sysbus.usart1 analyzer screen.

The Renode system screen will show some warnings related to unimplemented registers and unhandled writes. Exploring these issues was outside the scope of the exercise.
