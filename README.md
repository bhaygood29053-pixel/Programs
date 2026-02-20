# Programs

## XNT Auto-Compound Vault

Anchor program implementing an XNT-only production vault with:
- deposit/withdraw using internal share accounting,
- strategist-only compound flow,
- explicit pause flags,
- admin-managed caps.

See [SPEC.md](./SPEC.md) for the supported behavior and non-goals.

## Local build/test commands

### WSL / Linux
```bash
./scripts/build-test.sh
```

### Windows PowerShell
```powershell
./scripts/build-test.ps1
```

### Manual commands
```bash
yarn install
anchor build
anchor test --skip-build
```
