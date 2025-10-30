
# Micro:bit V2 template

A simple and ready-to-go template for Micro:bit V2 projects using Rust with Zed or VS Code.





## Requirements

### Cargo generate
Install cargo-generate if you haven’t already:
```htpt
  cargo install cargo-generate
```
### IDE setup:
#### Zed:
Configuration already provided, no additional setup required. 

#### VS Code:
Create or edit .vscode/settings.json:
```htpt
{
    "rust-analyzer.cargo-watch.allTargets": false,
    "rust-analyzer.cargo-watch.arguments": [
        "--target",
        "thumbv7m-none-eabi"
    ]
}
```

## Usage
#### Generate a new project from this template:
```htpt 
cargo generate --git https://github.com/lilyud420/microbitv2-rust-template.git
```

#### Build and flash to your board:

```htpt
cargo embed
```

## References

#### Hardware:
```htpt 
https://tech.microbit.org/hardware/#hardware-description
```
```htpt
https://docs.nordicsemi.com/bundle/nRF52833_PS_v1.6/resource/nRF52833_PS_v1.6.pdf
```
```htpt
https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF
```


#### Book:
```htpt
https://docs.rust-embedded.org/discovery-mb2/
```