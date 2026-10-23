Closes #

## Cosa cambia

## Come l'ho verificato

## Checklist
- [ ] Il titolo della PR segue Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`, `ci:`).
- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-features`
- [ ] `cargo deny check`
- [ ] Documentazione aggiornata (README, CONTRACTS.md, docs/metrics.md, …) dove serve.
- [ ] ADR collegato, se la PR applica una decisione di design.
- [ ] Nessun codice che piazza ordini, firma transazioni, gestisce chiavi o accede ai domini di Polymarket.
- [ ] Nessun segreto e nessun dato a livello di wallet nel diff (vedi la politica dei dati in SECURITY.md).
