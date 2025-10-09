# Run Examples

## Latest Test (Valid Signature with Full Verification)

```bash
cargo run --release -- "7321ED9434799FED...TRUNCATED...E1F1" "rExampleAddr123456789xrpL1234567890" "example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890"
```

Expected output:
```
✅ AUTHENTICATION SUCCESSFUL

Security checks passed:
  ✅ Address derivation matches (proves public key ownership)
  ✅ Challenge matches (prevents replay attacks)
  ✅ Cryptographic signature valid
```

## Previous Test

```bash
cargo run --release -- "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574473045022100C6C677D4D4B05CEBD97709AEBB2D9319271D43C9F7ABA1AF2DC1D40BF3ED6A1F02202249770ED2D857F0AFF6BA682DAA6D76F14D62FAE8B86A8D5F1BC645FC2AB4C881143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303031323736383A34653237383962662D396532382D343937372D626539312D6236613937316633346132313A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1" "rExampleAddr123456789xrpL1234567890"
```

## Quick Test

For faster testing during development:
```bash
cargo run -- "<hex_blob>" "<address>"
```

## Build Only

```bash
cargo build --release
```

Then run directly:
```bash
./target/release/signature-verify "<hex_blob>" "<address>"
```
