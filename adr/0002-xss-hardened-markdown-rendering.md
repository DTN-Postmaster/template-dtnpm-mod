# ADR 0002: Rendu Markdown durci contre le XSS et les injections de nonce

## Statut
Accepté

## Contexte
Les bundles DTN proviennent d'acteurs distants sur le réseau radioamateur maillé. Afficher du contenu non fiable dans l'interface web (Preact/Vite) expose à des attaques Cross-Site Scripting (XSS).

## Décision
1. Assainissement strict HTML (`escapeHtml`, `escapeAttribute`).
2. Remplacement des blocs de code et des liens par des jetons aléatoires basés sur un nonce cryptographique (`crypto.getRandomValues`) et des délimiteurs de contrôle `\x00` afin d'empêcher toute collision ou falsification.
3. Blocage par défaut des images distantes et des schémas d'URI dangereux (`javascript:`, `vbscript:`, `data:` non-raster).
4. Validation regex linéaire sans quantificateurs imbriqués pour prévenir les attaques ReDoS.

## Conséquences
- Protection OWASP Top 10 A03:2021 (Injection) au plus haut niveau.
- Immunité contre le vol de cookies de session et d'informations sensibles dans le navigateur.
