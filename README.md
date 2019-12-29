# ChessWar  

a referee program and AI for fight between chess AI

## Plateau  

```
8  |t|c|f|r|d|f|c|t|   blanc  
7  |p|p|p|p|p|p|p|p|  
6  | | | | | | | | |  
5  | | | | | | | | |  
4  | | | | | | | | |  
3  | | | | | | | | |  
2  |P|P|P|P|P|P|P|P|  
1  |T|C|F|R|D|F|C|T|   noir  

    A B C D E F G H
```

espace: vide  
p: pion  
t: tour  
c: cavalier  
f: fou  
r: roi  
d: dame  

blanc: minuscules  
noir: majuscules  

## programme de l'IA 

Notation: entre [ ] les caracteres au choix (ex: [ab] correspond à a ou b).  
entrée (2 lignes):  
- [bn] [10]  
- 64 caracteres bout à bout en une seule ligne correspondant à l'état du plateau lu de gauche à droite puis de haut en bas.  

Le [bn] correspond à blanc ou noir, et le [10] correspond à rock possible ou non (1: rock possible).  

sortie :  
- le movement choisi en 4 caractères représentant les coordonnées de départ et d'arrivé: lettre puis chiffre (ex: a2a3).  

## Jouer une partie  

Pour jouer une partie, executer le programme du jury (chess\_referee) en lui passant en arguments les deux programmes d'IA choisis (le premier argument correspond au joueur blanc).  

