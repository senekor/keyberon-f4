@_default:
    just --list --unsorted

flash:
    cargo objcopy --bin keyberon-f4 --release -- -O binary keyberon.bin
    dfu-util -w -d 0483:df11 -a 0 --dfuse-address 0x08000000:leave -D keyberon.bin
