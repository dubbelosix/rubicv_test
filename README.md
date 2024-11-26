### Install riscv tools
```
sudo apt install gcc-riscv64-unknown-elf
```
### Build binary
```
cargo build --release
riscv64-unknown-elf-objcopy -O binary --only-section=.text --only-section=.rodata --only-section=.eh_frame target/riscv32im-unknown-none-elf/release/rubicv_test output.bin
```

### examine with hexdump
```
hexdump -C output.bin
```

