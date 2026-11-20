# Jade for Horizon

Portage non officiel de [Jade](https://www.curseforge.com/minecraft/mc-mods/jade) (Snownee) pour [Horizon](https://playhorizon.dev) (jeu 26.3, API de mod 0.2). Le Jade d'origine est un mod Java NeoForge, qui ne peut pas tourner dans Horizon : ce mod réécrit ses fonctions principales en Rust (WebAssembly), avec le SDK officiel.

## Ce qu'il affiche

En haut de l'écran, une infobulle sur ce que vise le réticule :

- **Bloc** : icône, nom (dans la langue du jeu), mod d'origine, et selon le bloc :
  - croissance des cultures (blé, carottes, pommes de terre, betteraves, tiges de melon et de citrouille, cacao, verrues du Nether, baies sucrées, torchflower, pitcher ; et toute culture moddée qui a une propriété `age`) : « Croissance : 57 % » ou « Mûr » ;
  - miel d'une ruche (0 à 5), niveau d'un composteur (0 à 8), charges d'une ancre de réapparition ;
  - cuisson d'un four, fumoir ou haut fourneau, infusion d'un alambic ;
  - si l'objet en main permet de récolter le bloc (pour les blocs qui ne lâchent rien sans le bon outil) ;
  - une barre de progression pendant le minage.
- **Entité** : nom, points de vie (❤ 18 / 20), mod d'origine, et pour les animaux : temps avant de devenir adulte, temps avant de pouvoir se reproduire, mode amoureux, temps avant le prochain œuf (poules).

La touche **0 du pavé numérique** affiche ou masque l'infobulle (modifiable dans Options > Commandes).

Les points de vie, l'âge des animaux et la progression des fours et alambics ne sont connus que du serveur. Le mod les demande quelques fois par seconde, seulement pour une cible à moins de 24 blocs. Sur un serveur sans ce mod, l'infobulle marche quand même, sans ces lignes.

Pas encore portés depuis Jade : contenu des coffres et fours, statistiques des chevaux, propriétaire des animaux apprivoisés, thèmes et écran de configuration.

## Compiler et installer

```powershell
.\build.ps1
```

Comme pour Pier's Farm : compilation dans WSL (Ubuntu), copie dans `%APPDATA%\horizon\horizon-mods\`, activation dans `horizon-mods.txt`. Relancer Horizon ensuite.

## Licence et crédits

CC-BY-NC-SA-4.0 (voir [LICENSE](LICENSE) et [NOTICE](NOTICE)), comme Jade. Usage non commercial uniquement.

Jade est l'œuvre de [Snownee et ses contributeurs](https://github.com/Snownee/Jade), sous licence CC BY-NC-SA 4.0. Ce portage non officiel est une adaptation (réécriture en Rust pour Horizon, avec seulement une partie des fonctions de Jade). Il ne contient aucun asset ni donnée de Minecraft : les icônes et les noms sont lus dans le jeu du joueur. Il n'est pas affilié à Snownee ni au projet Horizon.
