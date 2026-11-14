# Politique de Sécurité - template-dtnpm-mod

Le projet `template-dtnpm-mod` applique les exigences du **Cyber Resilience Act (CRA)** européen et du référentiel **OWASP Top 10**.

## Versions supportées

| Version | Supportée |
| ------- | --------- |
| 0.1.x   | ✅ Oui    |

## Principes de Sécurité
- **Authentification & BPSec (RFC 9172)** : Mode strict par défaut. Tout bundle non authentifié ou falsifié est rejeté à la frontière du module.
- **Gestion des Secrets** : Les jetons Bearer doivent impérativement être passés par fichier via `--token-file` ou `DTN_HDY_TOKEN_FILE` afin d'éviter toute fuite dans `/proc/<pid>/cmdline`.
- **Mitigation XSS / DOM** : Tout contenu Markdown provenant de pairs distants est rendu via l'utilitaire durci `renderMarkdown` utilisant des nonces cryptographiques éphémères.
- **Absence de panique** : Code de production garanti sans `unwrap()` ni `panic!`.

## Signalement d'une Vulnérabilité

Pour signaler une vulnérabilité de sécurité, merci de contacter l'équipe de sécurité à :
`security@dtn-postmaster.org` ou d'ouvrir un avis de sécurité privé (GitHub Security Advisory).
