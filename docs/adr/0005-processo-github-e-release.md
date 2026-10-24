---
stato: accettato
data: 2026-10-23
decisori: Daniele Marino
issue: "#13"
---

# 0005. Processo di lavoro su GitHub e release

## Contesto e problema

La specifica chiede che tutto il lavoro passi da GitHub: issue per ogni attività, PR collegate, merge squash, Conventional
Commits, una release per fase con changelog generato, Dependabot e cargo-deny. Servono convenzioni precise e verificabili.

## Decisione

- **Issue**: una per attività, creata con i form `feature`, `bug`, `design-decision`, `research`; label `fase:*`, `type:*`,
  `area:*`, `prio:*` e milestone della fase o del checkpoint. Le issue di una fase si creano dopo l'approvazione del piano.
- **Board** "Sedette" (Projects v2, pubblica): Backlog → Design → Pronto → In corso → Review → Fatto.
- **Branch**: `<tipo>/<numero>-<nome-breve>` da `main` aggiornato.
- **Commit e PR**: Conventional Commits; il titolo della PR diventa il commit squash ed è controllato dal job `pr-title`;
  la descrizione contiene `Closes #N`.
- **Merge**: solo squash, solo con tutti i check verdi (ruleset su `main`, ADR 0001); il branch viene cancellato.
- **Release**: a fine fase una PR `chore(release): vX.Y.Z` aggiorna `CHANGELOG.md` con git-cliff; dopo il merge si crea il
  tag sul commit di merge e il workflow di release pubblica la GitHub Release. Versioni: `v0.1.0` per la Fase 0, `v0.2.0`
  per la Fase 1, e così via.
- **Manutenzione**: Dependabot settimanale per cargo e GitHub Actions (minor e patch raggruppate); `cargo deny` in CI.
- **Configurazione del repository**: descritta in `scripts/github/bootstrap.sh`, idempotente e con `--dry-run`.
- **Merge da parte di Claude**: autorizzato dall'utente ("procedi automaticamente"); la piattaforma impone comunque PR e
  check verdi.

### Conseguenze

- Positive: storia lineare e leggibile; changelog generato; configurazione riproducibile.
- Negative: alcune automazioni della board si configurano solo dall'interfaccia web (passaggi manuali in CONTRIBUTING.md).

## Riferimenti

- Specifica, sezione "Gestione del progetto su GitHub".
- [CONTRIBUTING.md](../../CONTRIBUTING.md), [scripts/github/bootstrap.sh](../../scripts/github/bootstrap.sh).
