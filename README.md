# kindle-tool
rust & binrw-based parser for kindle firmware updates

## usage

### **kindle inspect** `<INPUT_FILE>`
display the metadata of a firmware file [aliases: info]

**arguments**:
- `<INPUT_FILE>`: kindle firmware (.bin) file to inspect

### **kindle dump** `<INPUT_FILE>` `[OUTPUT_FILE]`
extract the deobfuscated tar.gz payload from a firmware file [aliases: convert]

**arguments**:
- `<INPUT_FILE>`: kindle firmware (.bin) file to process
- `[OUTPUT_FILE]`: output file for the .tar.gz payload (*default: stdout*)

### **kindle dm** `[INPUT_FILE]` `[OUTPUT_FILE]`
deobfuscate a data stream.

**arguments**:
- `[INPUT_FILE]`: input file to deobfuscate (*default: stdin*)
- `[OUTPUT_FILE]`: file to write deobfuscated data to (*default: stdout*)

## build
`kindle-tool` uses cargo for dependencies and builds: `cargo build --release`

## credits
- 2012–2025 [NiLuJe/KindleTool](https://github.com/NiLuJe/KindleTool/tree/master): updated KindleTool by NiLuJe
- 2011–2012 [yifanlu/KindleTool](https://github.com/yifanlu/KindleTool): original KindleTool by Yifan Lu
- 2009-2011 [kindle_update_tool.py](https://www.mobileread.com/forums/showpost.php?p=1805443): by Igor Skochinsky & Jean-Yves Avenard